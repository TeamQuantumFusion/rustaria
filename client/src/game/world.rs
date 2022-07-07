use std::ops::{Deref, DerefMut};
use rsa_core::api::Core;
use rsa_core::debug::DebugRendererImpl;
use rsa_core::err::ext::AuditExt;
use rsa_core::std::FxHashSet;
use rsa_core::err::Result;
use rsa_core::math::{Rect, size2};
use rsa_network::client::{ClientNetwork, ClientSender};
use rsa_world::ty::ChunkPos;
use rsa_world::{CHUNK_SIZE_F32, ClientBoundWorldPacket, ServerBoundWorldPacket, World};
use rsa_world::rpc::WorldRPC;
use rsa_client_player::PlayerSystem;
use crate::ClientRPC;

pub struct ClientWorld {
	pub inner: World,

	requested_chunks: FxHashSet<ChunkPos>,
}

impl ClientWorld {
	pub fn new(inner: World) -> ClientWorld {
		ClientWorld {
			inner,
			requested_chunks: Default::default(),
		}
	}

	pub fn tick_client(
		&mut self,
		core: &Core,
		rpc: &ClientRPC,
		player: &PlayerSystem,
		network: &mut ClientSender<ServerBoundWorldPacket>,
		debug: &mut impl DebugRendererImpl,
	) -> Result<()> {
		let pos = player.get_pos();
		let rect = Rect::new(pos.to_point(), size2(0.0, 0.0))
			.inflate(4.0 * CHUNK_SIZE_F32, 4.0 * CHUNK_SIZE_F32)
			.scale(1.0 / CHUNK_SIZE_F32, 1.0 / CHUNK_SIZE_F32)
			.round_out();
		let y_min = rect.origin.y as i64;
		let y_max = rect.origin.y as i64 + rect.size.height as i64;
		let x_min = rect.origin.x as i64;
		let x_max = rect.origin.x as i64 + rect.size.width as i64;
		for y in y_min..y_max {
			for x in x_min..x_max {
				if let Ok(pos) = ChunkPos::try_from((x, y)) {
					if !self.chunks.contains(pos) && !self.requested_chunks.contains(&pos) {
						network.send(ServerBoundWorldPacket::RequestChunk(pos))?;
						self.requested_chunks.insert(pos);
					}
				}
			}
		}
		self.inner
			.tick(core, &rpc.world, debug)
			.wrap_err("Ticking client world.")?;
		Ok(())
	}

	pub(crate) fn packet(&mut self, rpc: &WorldRPC, packet: ClientBoundWorldPacket) -> Result<()> {
		match packet {
			ClientBoundWorldPacket::Chunk(chunk_pos, chunk) => {
				self.inner.chunks.insert(chunk_pos, chunk);
				self.requested_chunks.remove(&chunk_pos);
			}
			ClientBoundWorldPacket::SetBlock(pos, layer_id, block_id) => {
				self.place_block(rpc, pos, layer_id, block_id);
			}
			ClientBoundWorldPacket::SpawnEntity(entity, id) => {
				self.inner.entities.storage.insert(rpc, entity, id);
			}
			ClientBoundWorldPacket::UpdateEntity(packet) => {
				self.inner.entities.packet(&packet);
			}
		}
		Ok(())
	}
}

impl Deref for ClientWorld {
	type Target = World;

	fn deref(&self) -> &Self::Target { &self.inner }
}

impl DerefMut for ClientWorld {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}
