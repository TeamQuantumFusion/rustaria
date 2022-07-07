//! # Rustaria Common Library
//! Contains core types that are used across a lot of rustaria modules.
use semver::Version;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::{
	api::{
		plugin::{Plugin, Plugins},
		reload::Reload,
		stargate::Stargate,
	},
	blake3::Blake3Hash,
	log::{info, LevelFilter},
	thread_pool::{ThreadPool, ThreadPoolBuilder}
};

pub mod api;
pub mod blake3;
pub mod debug;
pub mod ty;

pub mod err {
	pub use anyways::*;
}
pub mod log {
	pub use log::*;
}
pub mod math {
	pub use euclid::*;
}

pub mod aabb;

pub mod num {
	pub use num::*;
}

pub mod std {
	pub use fxhash::*;
}

pub mod thread_pool {
	pub use rayon::*;
}

pub mod sync {
	pub use crossbeam::*;
	pub use parking_lot::*;
}

// Constants
pub const TPS: usize = 60;
pub const KERNEL_VERSION: Version = Version::new(0, 0, 1);

static mut INITILIZED: bool = false;
pub fn initialize() -> err::Result<()> {
	unsafe {
		if !INITILIZED {
			INITILIZED = true;
			TermLogger::init(
				LevelFilter::Trace,
				Config::default(),
				TerminalMode::Mixed,
				ColorChoice::Auto,
			)?;
			info!(
				"Logging initialized successfully for Rustaria {}",
				KERNEL_VERSION.to_string()
			);
		}
	}
	Ok(())
}
