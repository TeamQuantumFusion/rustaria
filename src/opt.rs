//! Command-line parameters used in Rustaria, for both clients and dedicated servers.

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Specifies the verbosity/detail of the logging output.
    ///
    /// This comes in four verbosity levels:
    ///
    /// · `(unset)`: No debug information shown, least verbose
    ///
    /// · `-v`: Some debug information shown, especially ones pertaining to the game itself.
    ///
    /// · `-vv`: Almost all debug information shown, including trace-level debug details.
    ///
    /// · `-vvv`: All debug information shown. This includes (unfortunately) a logging spam
    ///    that logs on _every_ single frame.
    ///    Output under this verbosity setting is practically _unusable_, and should only be
    ///    relied upon in desperate scenarios.
    #[structopt(short = "v", parse(from_occurrences = Verbosity::from_occurrences))]
    pub verbosity: Verbosity,

    /// The directory Rustaria find its configuration file and run in.
    #[structopt(long = "run_dir", parse(from_os_str), default_value = ".")]
    pub run_dir: PathBuf,
}

/// The verbosity setting for the logging output, set in the form of [command-line parameters](Opt).
///
/// It is primarily used in the [`init`] method to initialize an [`EnvFilter`].
/// The exact parameters that corresponds with each level can also be seen there.
///
/// [`init`]: crate::init
/// [`EnvFilter`]: tracing_subscriber::filter::EnvFilter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Only info-level information shown, least verbose.
    ///
    /// Corresponds to a lack of a verbosity flag in the [command-line parameters](Opt).
    Normal,
    /// Some debug-level information shown, especially ones pertaining to the game itself.
    ///
    /// Corresponds to one (1) verbosity flag (`-v`) in the [command-line parameters](Opt).
    Verbose,
    /// Almost all debug information shown, including trace-level debug details.
    ///
    /// Corresponds to two (2) verbosity flags (`-vv`) in the [command-line parameters](Opt).
    VeryVerbose,
    /// All debug information shown.
    ///
    /// **CAUTION**: This includes (unfortunately) a logging spam that logs on _every_ single frame.
    /// Output under this verbosity setting is practically _unusable_, and should only be
    /// relied upon in desperate scenarios.
    ///
    /// Corresponds to three (3) verbosity flags (`-vvv`) in the [command-line parameters](Opt).
    VeryVeryVerbose,
}
impl Verbosity {
    fn from_occurrences(n: u64) -> Self {
        match n {
            0 => Self::Normal,
            1 => Self::Verbose,
            2 => Self::VeryVerbose,
            _ => Self::VeryVeryVerbose,
        }
    }
}
