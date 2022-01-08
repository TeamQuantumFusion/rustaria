use eyre::Result;
use time::macros::format_description;
use tracing::{error, info};
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

#[tokio::main]
async fn main() -> Result<()> {
    init();
    info!("Rustaria v{}", env!("CARGO_PKG_VERSION"));


    let mut plugins_dir = std::env::current_dir()?;
    plugins_dir.push("plugins");

    let mut plugins = PluginLoader::new()?;
    // TODO sensei this is your problem now.
    plugins.scan_and_load_plugins_internal(&plugins_dir).await?;
    plugins.bootstrap()?;
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

