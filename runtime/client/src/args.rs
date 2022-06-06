use std::collections::{HashMap, HashSet};
use rsa_core::logging::LevelFilter;

pub struct ClientOptions {
	pub mode: ClientMode,
	pub logging: LevelFilter,
}

pub enum ClientMode {
	Normal,
	/// If the client-old should run in a way that is optimal for plugin development.
	Development,
	/// KernelDev is only for running when you have the rustaria rust project opened.
	KernelDev,
}
const MODE_ARGS: ([(&str, ClientMode); 2], ClientMode) = (
	[
		("--dev", ClientMode::Development),
		("--kernel", ClientMode::KernelDev),
	],
	ClientMode::Normal,
);


const LOG_ARGS: ([(&str, LevelFilter); 4], LevelFilter) = (
	[
		("--logging=error", LevelFilter::Error),
		("--logging=warn", LevelFilter::Warn),
		("--logging=debug", LevelFilter::Debug),
		("--logging=trace", LevelFilter::Trace),
	],
	LevelFilter::Info,
);

// ok yes clap exists but its huge af.
impl ClientOptions {
	pub fn new() -> ClientOptions {
		let args: HashSet<String> = std::env::args().collect();
		ClientOptions {
			mode:  Self::get_arg(&args, MODE_ARGS),
			logging:  Self::get_arg(&args, LOG_ARGS)
		}
	}

	fn get_arg<const L: usize, T>(args: &HashSet<String>, values: ([(&'static str, T); L], T)) -> T {
		for (arg, value) in values.0 {
			if args.contains(arg) {
				return value;
			}
		}

		// default
		values.1
	}
}