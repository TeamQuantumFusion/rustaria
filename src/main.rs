use std::{collections::HashSet, env};

use eyre::Result;
use mlua::Lua;
use time::macros::format_description;
use tracing::info;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt::time::UtcTime, prelude::*};

use rustaria::{api, plugin::PluginLoader};
use rustaria::api::Prototype;
use rustaria::registry::Registry;

#[tokio::main]
async fn main() -> Result<()> {
    let args: HashSet<_> = env::args().collect();
    init(args.contains("--debug"))?;

    info!("Rustaria v{}", env!("CARGO_PKG_VERSION"));

    let mut plugins_dir = env::current_dir()?;
    plugins_dir.push("plugins");

    let lua = Lua::new();

    let (send, mut rec) = tokio::sync::mpsc::unbounded_channel();
    api::register_rustaria_api(&lua, send.clone())?;

    let loader = PluginLoader { plugins_dir };
    let plugins = loader.scan_and_load_plugins(&lua).await?;
    plugins.init()?;

    while let Some(Prototype::Tile(name, data)) = rec.recv().await {
        info!("Registering Prototype {} value {:?}", name, data )
    }


    let registry = Registry::new();

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
