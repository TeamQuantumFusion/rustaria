use std::{collections::HashSet, env};

use eyre::{ContextCompat, Error, Result};
use mlua::Lua;
use time::macros::format_description;
use tracing::info;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt::time::UtcTime, prelude::*};

use rustaria::{api, plugin};
use rustaria::api::PrototypeRequest;
use rustaria::chunk::Chunk;
use rustaria::player::Player;
use rustaria::registry::{Id, Registry, RegistryStack, Tag};
use rustaria::world::World;

#[tokio::main]
async fn main() -> Result<()> {
    let args: HashSet<_> = env::args().collect();
    init(args.contains("--debug"))?;

    info!("Rustaria v{}", env!("CARGO_PKG_VERSION"));

    let mut plugins_dir = env::current_dir()?;
    plugins_dir.push("plugins");

    // init lua api
    let lua = Lua::new();
    let mut receiver = api::register_rustaria_api(&lua)?;

    let plugins = plugin::scan_and_load_plugins(&plugins_dir, &lua).await?;

    // call initPath files
    plugins.init(&lua)?;

    // register all prototypes
    let mut tile_registry = Registry::new();
    let mut wall_registry = Registry::new();
    while let Some(prototype) = receiver.recv().await {
        match prototype {
            PrototypeRequest::Tile(id, pt) => tile_registry.register(id, pt),
            PrototypeRequest::Wall(id, pt) => wall_registry.register(id, pt),
        };
    }

    let stack = RegistryStack {
        tile: tile_registry,
        wall: wall_registry,
    };

    // create runtime
    let air_tile = stack.tile.get_id(&Tag::parse("rustaria-core:air")?).wrap_err("Could not find air tile?")?;
    let air_wall = stack.wall.get_id(&Tag::parse("rustaria-core:air")?).wrap_err("Could not find air wall?")?;
    let empty_chunk = Chunk::new(&stack, air_tile, air_wall).wrap_err("Could not create empty Chunk")?;
    let mut world = World::new((2, 2), vec![
        empty_chunk, empty_chunk,
        empty_chunk, empty_chunk,
    ]).map_err(Error::msg)?;
    world.player_join(Player::new(0.0, 0.0, "dev".to_string()));

    Ok(())
}

fn init(debug: bool) -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let timer = UtcTime::new(format_description!(
        "[hour]:[minute]:[second].[subsecond digits:3]"
    ));
    let format = tracing_subscriber::fmt::format()
        .with_timer(timer)
        .compact();
    let fmt_layer = tracing_subscriber::fmt::layer().event_format(format);

    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| {
        EnvFilter::try_new({
            if debug {
                "debug"
            } else {
                "info"
            }
        })
    })?;

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
