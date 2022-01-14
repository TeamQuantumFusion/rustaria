use eyre::{eyre, Result};
use std::{env, path::PathBuf};
use structopt::StructOpt;
use tracing::info;

use rustaria::api::{self, LuaRuntime};
use rustaria::chunk::Chunk;
use rustaria::player::Player;
use rustaria::registry::{Tag};
use rustaria::world::World;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustaria-server", about = "The serverside face of Rustaria")]
struct Opt {
    /// Activate debug mode (equivalent to setting RUST_LOG to "debug")
    #[structopt(long)]
    debug: bool,

    /// Plugin directory. Defaults to `./plugins`.
    #[structopt(long = "plugins_dir", parse(from_os_str), default_value = "plugins")]
    plugins_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    rustaria::init_console(opt.debug)?;

    info!("Rustaria Dedicated Server v{}", env!("CARGO_PKG_VERSION"));
    let lua = LuaRuntime::new();
    let stack = api::launch_rustaria_api(opt.plugins_dir, &lua).await?;

    // create runtime
    let air_tile = stack
        .tiles
        .get_id(&Tag::parse("rustaria-core:air")?)
        .ok_or_else(|| eyre!("Could not find air tile"))?;
    let air_wall = stack
        .walls
        .get_id(&Tag::parse("rustaria-core:air")?)
        .ok_or_else(|| eyre!("Could not find air wall"))?;
    let empty_chunk = Chunk::new(&stack, air_tile, air_wall)
        .ok_or_else(|| eyre!("Could not create empty chunk"))?;
    let mut world = World::new(
        (2, 2),
        vec![empty_chunk, empty_chunk, empty_chunk, empty_chunk],
    )?;
    world.player_join(Player::new(0.0, 0.0, "dev".to_string()));

    Ok(())
}
