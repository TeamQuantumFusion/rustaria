use std::ops::{Deref, DerefMut};

use euclid::{size2, Rect};
use eyre::{Result, WrapErr};
use fxhash::FxHashSet;
use rustaria::{
	api::Api,
	debug::DebugRendererImpl,
	network::ClientNetwork,
	ty::chunk_pos::ChunkPos,
	world::{
		chunk::{Chunk, CHUNK_SIZE_F32},
		ClientBoundWorldPacket, ServerBoundWorldPacket, World,
	},
};

use crate::{ClientApi, PlayerSystem};

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
		api: &ClientApi,
		player: &PlayerSystem,
		network: &mut ClientNetwork,
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
		self.inner.tick(api, debug).wrap_err("Ticking client world.")?;
		Ok(())
	}

	pub(crate) fn packet(
		&mut self,
		api: &Api,
		packet: ClientBoundWorldPacket,
		debug: &mut impl DebugRendererImpl,
	) -> Result<()> {
		match packet {
			ClientBoundWorldPacket::Chunk(chunk_pos, chunk) => {
				self.inner.chunks.insert(chunk_pos, chunk);
				self.requested_chunks.remove(&chunk_pos);
			}
			ClientBoundWorldPacket::SetBlock(pos, layer_id, block_id) => {
				self.place_block(api, pos, layer_id, block_id);
			}
			ClientBoundWorldPacket::SpawnEntity(entity, id) => {
				self.inner.entities.storage.insert(api, entity, id);
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

pub enum ChunkRequestStatus {
	Requested,
	Received(Chunk),
}
