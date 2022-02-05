use eyre::Result;
use mlua::Lua;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;
use tracing::{debug, info};

use rustaria::api::Rustaria;
use rustaria::chunk::Chunk;
use rustaria::network::packet::ClientPacket::ILoveYou;
use rustaria::network::packet::ServerPacket;
use rustaria::network::server::ServerNetwork;
use rustaria::opt::Verbosity;
use rustaria::world::World;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustaria-server", about = "The serverside face of Rustaria")]
struct Opt {
    #[structopt(flatten)]
    inner: rustaria::opt::Opt,
}

#[tokio::main]
async fn main() -> Result<()> {
    rustaria::init(Verbosity::VeryVerbose)?;


    let server_addr = SocketAddr::from_str("127.0.0.1:42069").unwrap();
    let mut server = rustaria::network::server::Server {
        network: ServerNetwork::new(Some(server_addr), false)
    };
    println!("Server launched");

    loop {
        std::thread::sleep(Duration::from_millis(10));
        server.network.tick();
        for (source, packet) in server.network.receive() {
            if let ILoveYou = packet {
                server.network.send(&source, &ServerPacket::FuckOff);
            }
        }
    }

//     let opt = Opt::from_args();
//     debug!(?opt, "Got command-line args");
//
//     rustaria::init(opt.inner.verbosity)?;
//
//     info!("Rustaria Dedicated Server v{}", env!("CARGO_PKG_VERSION"));
//     let lua = Lua::new();
//     let api = Rustaria::new(opt.inner.plugins_dir, &lua).await?;
//
//     // create runtime
//     let air_tile = api
//         .tiles
//         .get_id_from_tag(&"rustaria:air".parse()?)
//         .expect("Could not find air tile");
//     let air_wall = api
//         .walls
//         .get_id_from_tag(&"rustaria:air".parse()?)
//         .expect("Could not find air wall");
//     let empty_chunk = Chunk::new(&api, air_tile, air_wall).expect("Could not create empty chunk");
//     let mut world = World::new(
//         (2, 2),
//         vec![empty_chunk, empty_chunk, empty_chunk, empty_chunk],
//     )?;
//
//     let player = api
//         .entities
//         .get_from_tag(&"rustaria:player".parse()?)
//         .expect("Could not find player entity");
//     player.spawn(&mut world);
//
//     debug!("{:?}", world.comps);

    Ok(())
}
