#![feature(negative_impls)]

use std::fs::File;

use color_eyre::config::Theme;
use color_eyre::owo_colors::Style;
use log::{Level, LevelFilter};
// Imports
use simplelog::{
	Color, ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelPadding, TargetPadding,
	TerminalMode, TermLogger, WriteLogger,
};

pub mod api;
pub mod blake3;
pub mod plugin;
pub mod registry;
pub mod settings;
pub mod ty;
pub mod hook;

pub mod logging {
	pub use log::*;
}

pub mod error {
	pub use eyre::*;
	pub use thiserror::Error;
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

static mut INITIALIZED: bool = false;
pub fn initialize(level: LevelFilter) -> eyre::Result<()> {
	if unsafe { !INITIALIZED } {
		unsafe {
			INITIALIZED = true;
		}
		std::env::set_var("RUST_BACKTRACE", "1");
		color_eyre::config::HookBuilder::new()
			.theme(
				Theme::new()
					.file(Style::new().white())
					.line_number(Style::new().purple())
					.spantrace_target(Style::new().green())
					.spantrace_fields(Style::new().green())
					.active_line(Style::new().purple())
					.error(Style::new().bright_white().bold())
					.help_info_note(Style::new().cyan())
					.help_info_warning(Style::new().yellow())
					.help_info_suggestion(Style::new().blue())
					.help_info_error(Style::new().red())
					.dependency_code(Style::new().blue())
					.crate_code(Style::new().cyan())
					.code_hash(Style::new().bright_black())
					.panic_header(Style::new().red())
					.panic_message(Style::new().white())
					.panic_file(Style::new().red())
					.panic_line_number(Style::new().purple())
					.hidden_frames(Style::new().red()),
			)
			.install()?;

		CombinedLogger::init(vec![
			TermLogger::new(
				level,
				ConfigBuilder::new()
					//.set_time_format_str("\x1b[37m%T")
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
	}
	Ok(())
}
