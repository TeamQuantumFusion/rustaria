use std::fs::File;

// Imports
pub use log::*;
use simplelog::{ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, WriteLogger, Color, TargetPadding};
pub use uuid::Uuid;

pub mod blake3;
pub mod ty;

pub fn initialize() -> eyre::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            ConfigBuilder::new()
                .set_level_padding(LevelPadding::Off)
                .set_target_level(LevelFilter::Error)
                .set_target_padding(TargetPadding::Left(2))
                .set_level_color(Level::Trace, Some(Color::Magenta))
                .set_level_color(Level::Debug, Some(Color::Blue))
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
            File::create("my_rust_binary.log").unwrap(),
        ),
    ])?;

    Ok(())
}

pub fn uuid() -> Uuid {
    Uuid::new_v4()
}
