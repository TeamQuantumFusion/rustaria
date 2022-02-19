#![allow(clippy::new_without_default)]

use std::env;

use time::macros::format_description;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use opt::Verbosity;

pub const KERNEL_VERSION: (u8, u8, u8) = (0, 0, 1);
pub const UPS: u32 = 20; // updates per second
pub const MS_PER_UPDATE: u64 = (1000.0 / UPS as f32) as u64;

pub mod hash;
pub mod opt;
pub mod player;
pub mod types;

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

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}

// pub struct Server {
//     world: World,
//     pub network: ServerNetwork,
//
//     last_tick: Instant,
// }
//
// impl Server {
//     pub fn new(world: World, network: ServerNetwork) -> Server {
//         Server {
//             world,
//             network,
//             last_tick: Instant::now(),
//         }
//     }
//
//     fn tick_internal(&mut self, rustaria: &Rustaria) -> eyre::Result<()> {
//         self.network.tick();
//         for (source, packet) in self.network.receive(rustaria) {
//             match packet {
//                 ClientPacket::ILoveYou => {}
//                 ClientPacket::RequestChunk(pos) => {
//                     if let Some(chunk) = self.world.get_chunk(pos) {
//                         match ChunkPacket::new(chunk) {
//                             Ok(packet) => {
//                                 self.network
//                                     .send(
//                                         &source,
//                                         &ServerPacket::Chunk {
//                                             data: Box::new(packet),
//                                         },
//                                         PacketDescriptor {
//                                             priority: PacketPriority::Reliable,
//                                             order: PacketOrder::Unordered,
//                                         },
//                                     )
//                                     .unwrap();
//                             }
//                             Err(err) => {
//                                 warn!("Could not send chunk  {}", err)
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         Ok(())
//     }
//
//     pub fn tick(&mut self, rustaria: &Rustaria) -> eyre::Result<()> {
//         loop {
//             let duration = self.last_tick.elapsed();
//             match duration.as_secs() {
//                 60.. => bail!("Server ran 1 minute behind. Closing server."),
//                 secs @ 5..=59 => warn!("Server running {secs} behind"),
//                 _ => {}
//             }
//             if duration.as_millis() < MS_PER_UPDATE as u128 {
//                 break; // done
//             }
//             self.tick_internal(rustaria)?;
//             self.last_tick += Duration::from_millis(MS_PER_UPDATE);
//         }
//         Ok(())
//     }
// }