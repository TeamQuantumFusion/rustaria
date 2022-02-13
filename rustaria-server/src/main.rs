use eyre::Result;
use mlua::Lua;
use std::net::SocketAddr;
use std::str::FromStr;
use structopt::StructOpt;
use tracing::debug;

use rustaria::api::Rustaria;
use rustaria::chunk::Chunk;
use rustaria::network::server::ServerNetwork;
use rustaria::world::World;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustaria-server", about = "The serverside face of Rustaria")]
struct Opt {
    #[structopt(flatten)]
    inner: rustaria::opt::Opt,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    debug!(?opt, "Got command-line args");
    rustaria::init(opt.inner.verbosity)?;

    let lua = Lua::new();
    let api = Rustaria::new(opt.inner.plugins_dir, &lua)?;

    let server_addr = SocketAddr::from_str("127.0.0.1:42069").unwrap();
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

    let mut server = rustaria::Server::new(world, ServerNetwork::new(Some(server_addr), false));
    println!("Server launched");

    loop {
        server.tick(&api).unwrap();
    }
}
