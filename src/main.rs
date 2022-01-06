use eyre::Result;
use time::macros::format_description;
use tracing::{error, info};
use tracing_subscriber::{fmt::time::UtcTime, prelude::*, EnvFilter};


use crate::{plugin::PluginLoader};


mod api;
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

    let mut plugins = PluginLoader::new()?;
    let mut plugins_dir = std::env::current_dir()?;
    plugins_dir.push("plugins");
    plugins.scan_and_load_plugins(&plugins_dir).await?;
    plugins.run()?;

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

