#![feature(negative_impls)]

use std::fs::File;

use log::{Level, LevelFilter};
// Imports
use simplelog::{
	Color, ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelPadding, TargetPadding,
	TermLogger, TerminalMode, WriteLogger,
};
pub use uuid::Uuid;

pub mod blake3;
pub mod rcl;
pub mod ty;
pub mod settings;

pub mod logging {
	pub use log::*;
}

pub mod error {
	pub use eyre::*;
}

pub mod math {
	pub use euclid::*;

	pub struct WorldSpace;
	pub struct ScreenSpace;
	pub struct AtlasSpace;

	pub fn limit(value: f32, limit: f32) -> f32 {
		value.clamp(-limit, limit)
	}
}

pub fn initialize() -> eyre::Result<()> {
	std::env::set_var("RUST_BACKTRACE", "1");
	color_eyre::install()?;

	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			ConfigBuilder::new()
				.set_time_format_str("\x1b[37m%T")
				.set_level_padding(LevelPadding::Off)
				.set_target_level(LevelFilter::Error)
				.set_target_padding(TargetPadding::Left(2))
				.set_level_color(Level::Trace, Some(Color::Rgb(255, 0, 255)))
				.set_level_color(Level::Debug, Some(Color::Cyan))
				.set_level_color(Level::Info, Some(Color::Green))
				.set_level_color(Level::Warn, Some(Color::Yellow))
				.set_level_color(Level::Error, Some(Color::Red))
				.build(),
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(
			LevelFilter::Info,
			Config::default(),
			File::create("rustaria.log").unwrap(),
		),
	])?;

	Ok(())
}

pub fn uuid() -> Uuid {
	Uuid::new_v4()
}
