pub mod config;

use eyre::Result;
use mlua::Lua;
use rustaria::Server;
use structopt::StructOpt;
use tracing::{debug, info};

use rustaria::api::Rustaria;
use rustaria::chunk::Chunk;
use rustaria::network::server::ServerNetwork;
use rustaria::world::World;

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
    let lua = Lua::new();
    let api = Rustaria::new(run_dir, &lua)?;

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
        server.tick(&api)?;
    }
}
