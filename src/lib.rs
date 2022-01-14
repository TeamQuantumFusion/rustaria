use std::env;
use time::macros::format_description;
use tracing::info;
use tracing_error::ErrorLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod api;
pub mod biome;
pub mod chunk;
pub mod entity;
pub mod gen;
pub mod physics;
pub mod player;
pub mod registry;
pub mod world;
pub mod plugin;

pub fn init_console(debug_mode: bool) -> eyre::Result<()> {
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
			if debug_mode {
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

	info!("Console with {}", debug_mode);

	Ok(())
}

