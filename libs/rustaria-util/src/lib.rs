// Re-exports
pub use eyre::*;
// Imports
use time::macros::format_description;
pub use tracing::*;
use tracing_error::ErrorLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
pub use uuid::Uuid;

pub mod blake3;
pub mod ty;

pub fn initialize() -> eyre::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let timer = UtcTime::new(format_description!(
        "[hour]:[minute]:[second].[subsecond digits:3]"
    ));
    let format = tracing_subscriber::fmt::format()
        .with_timer(timer)
        .compact();
    let fmt_layer = tracing_subscriber::fmt::layer().event_format(format);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(EnvFilter::try_new("debug,wgpu_hal=warn,wgpu_core=warn").unwrap())
        .with(ErrorLayer::default())
        .init();

    Ok(())
}

pub fn uuid() -> Uuid {
    Uuid::new_v4()
}
