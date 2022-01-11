use eyre::Result;
use mlua::Lua;
use std::{collections::HashSet, env};
use time::macros::format_description;
use tracing::info;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::time::UtcTime, prelude::*, EnvFilter};

use crate::plugin::PluginLoader;

mod api;
mod biome;
mod chunk;
mod entity;
mod gen;
mod physics;
mod player;
mod plugin;
mod registry;
mod world;

#[tokio::main]
async fn main() -> Result<()> {
    let args: HashSet<_> = env::args().collect();
    init(args.contains("--debug"))?;

    info!("Rustaria v{}", env!("CARGO_PKG_VERSION"));

    let mut plugins_dir = env::current_dir()?;
    plugins_dir.push("plugins");

    let lua = Lua::new();
    api::register_rustaria_api(&lua)?;
    let loader = PluginLoader { plugins_dir };
    let plugins = loader.scan_and_load_plugins(&lua).await?;
    plugins.init()?;

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
