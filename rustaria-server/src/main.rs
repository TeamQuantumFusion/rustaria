pub mod cmd;
pub mod config;

use std::path::Path;
use std::time::Duration;

use cmd::Command;
use crossbeam::channel::{unbounded, Receiver};
use crossbeam::select;
use eyre::Result;
use mlua::Lua;
use rustaria::api::loader::Loader;
use rustaria::api::plugin::Plugins;
use rustaria::api::Rustaria;
use rustaria::Server;
use structopt::StructOpt;
use tracing::{debug, error, info, trace, warn};

use rustaria::chunk::Chunk;
use rustaria::network::server::ServerNetwork;
use rustaria::world::World;

use crate::cmd::Commands;
use crate::config::Config;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustaria-server", about = "The serverside face of Rustaria")]
struct Opt {
    #[structopt(flatten)]
    inner: rustaria::opt::Opt,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    debug!(?opt, "Got command-line args");

    let Opt {
        inner: rustaria::opt::Opt { verbosity, run_dir },
    } = opt;
    rustaria::init(verbosity)?;

    let config = Config::from_file_or_default(&run_dir.join("config.toml"));
    let cmds = Commands::new();

    let (tx, rx) = unbounded();

    std::thread::spawn(move || server_loop(&run_dir, &config, rx).unwrap());

    let mut cmd = String::new();
    loop {
        let stdin = std::io::stdin();
        stdin.read_line(&mut cmd)?;

        match cmds.exec(cmd.trim()) {
            Some(cmd) => select! {
                send(tx, cmd) -> res => {
                    trace!(?res);
                    res?;
                },
                default(Duration::from_secs(5)) => {
                    warn!("Server has not received command for 5 seconds! Is the server thread down?");
                }
            },
            None => error!("Invalid command: {cmd}"),
        }
        cmd.clear();
    }
}

fn server_loop(run_dir: &Path, config: &Config, rx: Receiver<Command>) -> Result<()> {
    let lua = Lua::new();

    let mut loader = Loader::default();
    let mut api = Rustaria::default();

    let plugins_dir = run_dir.join("plugins");
    reload_plugins(&lua, &plugins_dir, &mut loader, &mut api)?;

    let air_tile = api
        .tiles
        .get_id_from_tag(&"rustaria:air".parse()?)
        .expect("Could not find air tile");
    let air_wall = api
        .walls
        .get_id_from_tag(&"rustaria:air".parse()?)
        .expect("Could not find air wall");
    let empty_chunk = Chunk::new(&api, air_tile, air_wall).expect("Could not create empty chunk");
    let world = World::new(
        (2, 2),
        vec![empty_chunk, empty_chunk, empty_chunk, empty_chunk],
    )?;

    let mut server = Server::new(
        world,
        ServerNetwork::new(Some(config.server.server_addr), false),
    );
    info!("Server listening on {}", config.server.server_addr);

    loop {
        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                Command::Reload => {
                    info!("Reloading plugins");
                    reload_plugins(&lua, &plugins_dir, &mut loader, &mut api)?;
                }
            }
        }
        server.tick(&api)?;
    }
}

fn reload_plugins(
    lua: &Lua,
    plugins_dir: &Path,
    loader: &mut Loader,
    api: &mut Rustaria,
) -> Result<()> {
    info!("Scanning for plugins in directory {plugins_dir:?}");
    let plugins = Plugins::load(plugins_dir)?;
    info!("Executing plugins");
    let outputs = loader.init(lua, &plugins)?;
    info!("Initializing API");
    api.reload(outputs);
    Ok(())
}
