use eyre::Result;
use mlua::Lua;
use time::macros::format_description;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt::time::UtcTime, prelude::*};

use crate::plugin::PluginLoader;

mod biome;
mod chunk;
mod entity;
mod gen;
mod physics;
mod plugin;
mod world;
mod player;
mod registry;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    info!("Rustaria v{}", env!("CARGO_PKG_VERSION"));


    let mut plugins_dir = std::env::current_dir()?;
    plugins_dir.push("plugins");

    let lua = Lua::new();
    api::register_rustaria_api(&lua)?;
    let loader = PluginLoader { plugins_dir };
    let plugins = loader.scan_and_load_plugins_internal(&lua).await?;
    plugins.init()?;
    Ok(())
}

fn init() {
    let timer = UtcTime::new(format_description!(
        "[hour]:[minute]:[second].[subsecond digits:3]"
    ));
    let format = tracing_subscriber::fmt::format()
        .with_timer(timer)
        .compact();
    let fmt_layer = tracing_subscriber::fmt::layer().event_format(format);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("`info` is not a valid EnvFilter... what?");
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();
}

