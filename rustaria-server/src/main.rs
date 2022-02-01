use eyre::Result;
use mlua::Lua;
use std::env;
use structopt::StructOpt;
use tracing::{debug, info};

use rustaria::api::Rustaria;
use rustaria::chunk::Chunk;
use rustaria::world::World;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustaria-server", about = "The serverside face of Rustaria")]
struct Opt {
    #[structopt(flatten)]
    inner: rustaria::opt::Opt,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    debug!(?opt, "Got command-line args");

    rustaria::init(opt.inner.verbosity)?;

    info!("Rustaria Dedicated Server v{}", env!("CARGO_PKG_VERSION"));
    let lua = Lua::new();
    let api = Rustaria::new(opt.inner.plugins_dir, &lua).await?;

    // create runtime
    let air_tile = api
        .tiles
        .get_id_from_tag(&"rustaria-core:air".parse()?)
        .expect("Could not find air tile");
    let air_wall = api
        .walls
        .get_id_from_tag(&"rustaria-core:air".parse()?)
        .expect("Could not find air wall");
    let empty_chunk = Chunk::new(&api, air_tile, air_wall).expect("Could not create empty chunk");
    let mut world = World::new(
        (2, 2),
        vec![empty_chunk, empty_chunk, empty_chunk, empty_chunk],
    )?;

    let player = api
        .entities
        .get_from_tag(&"rustaria-core:player".parse()?)
        .expect("Could not find player entity");
    player.spawn(&mut world);

    debug!("{:?}", world.comps);
    // world.player_join(Player::new(0.0, 0.0, "dev".to_string()));

    Ok(())
}
