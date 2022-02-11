extern crate core;

use std::env;
use std::ops::AddAssign;
use std::time::{Duration, Instant};

use eyre::Report;
use time::macros::format_description;
use tracing::{info, warn};
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::api::Rustaria;
use crate::network::packet::{ChunkPacket, ClientPacket, ServerPacket};
use crate::network::{PacketDescriptor, PacketOrder, PacketPriority};
use opt::Verbosity;

use crate::network::server::ServerNetwork;
use crate::world::World;

pub const KERNEL_VERSION: (u8, u8, u8) = (0, 0, 1);
pub const UPS: u32 = 20;

pub mod api;
mod blake3;
pub mod chunk;
pub mod comps;
pub mod entity;
pub mod network;
pub mod opt;
pub mod registry;
pub mod types;
pub mod world;
pub mod player;

/// Common initialization code for both Rustaria client and dedicated server.
/// This currently sets up [`color_eyre`] and [`tracing`].
pub fn init(verbosity: Verbosity) -> eyre::Result<()> {
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
        EnvFilter::try_new(match verbosity {
            Verbosity::Normal => "info,wgpu_hal=warn,wgpu_core=warn",
            Verbosity::Verbose => "debug,wgpu_hal=warn,wgpu_core=warn,naga=info",
            Verbosity::VeryVerbose => {
                "trace,wgpu_core::present=info,wgpu_core::device=info,wgpu_hal=info,naga=info"
            }
            Verbosity::VeryVeryVerbose => "trace",
        })
    })?;

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}

pub struct Server {
    world: World,
    pub network: ServerNetwork,

    last_tick: Instant,
}

impl Server {
    pub fn new(world: World, network: ServerNetwork) -> Server {
        Server {
            world,
            network,
            last_tick: Instant::now(),
        }
    }

    fn tick_internal(&mut self, rustaria: &Rustaria) -> eyre::Result<()> {
        self.network.tick();
        for (source, packet) in self.network.receive(rustaria) {
            match packet {
                ClientPacket::ILoveYou => {}
                ClientPacket::RequestChunk(pos) => {
                    if let Some(chunk) = self.world.get_chunk(pos) {
                        match ChunkPacket::new(chunk) {
                            Ok(packet) => {
                                self.network
                                    .send(
                                        &source,
                                        &ServerPacket::Chunk {
                                            data: Box::new(packet),
                                        },
                                        PacketDescriptor {
                                            priority: PacketPriority::Reliable,
                                            order: PacketOrder::Unordered,
                                        },
                                    )
                                    .unwrap();
                            }
                            Err(err) => {
                                warn!("Could not send chunk  {}", err)
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn tick(&mut self, rustaria: &Rustaria) -> eyre::Result<()> {
        while {
            {
                let duration = self.last_tick.elapsed();
                let seconds = duration.as_secs();
                if seconds > 60 {
                    return Err(Report::msg("Server ran 1 minute behind. Closing server."));
                } else if seconds > 5 {
                    warn!("Server running {} behind", seconds)
                }
                duration.as_millis()
            }
        } >= (1000.0 / UPS as f32) as u128
        {
            self.tick_internal(rustaria)?;
            self.last_tick += Duration::from_millis((1000.0 / UPS as f32) as u64);
        }

        Ok(())
    }
}
