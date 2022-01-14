use crate::plugin::PluginLoader;
use eyre::Result;
use mlua::Lua;
use rustaria::api;
use std::{env, path::PathBuf};
use structopt::StructOpt;
use time::macros::format_description;
use tracing::info;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::time::UtcTime, prelude::*, EnvFilter};

mod plugin;

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
    init(opt.debug)?;

    info!("Rustaria v{}", env!("CARGO_PKG_VERSION"));

    let lua = Lua::new();
    api::register_rustaria_api(&lua)?;
    let loader = PluginLoader {
        plugins_dir: opt.plugins_dir,
    };
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
