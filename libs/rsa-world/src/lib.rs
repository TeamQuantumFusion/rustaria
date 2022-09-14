//! # Rustaria World
//! Contains code for the world itself and all of its contents. Stuff like entities and blocks are here.
use hecs::Entity;
use chunk::block::ty::BlockType;
use rpc::WorldAPI;
use rsa_core::{
	api::Core,
	debug::DebugRendererImpl,
	err::{ext::AuditExt, Result},
};
use rsa_network::{server::ServerSender, Token};
use rsa_registry::Id;

use crate::{
	chunk::{Chunk, layer::ChunkLayerType, storage::ChunkStorage},
	entity::{
		EntityWorld,
		prototype::EntityType,
		system::network::{EntityComponentPacket, EntityPacket},
	},
	spread::SpreaderSystem,
	ty::{BlockPos, ChunkPos},
};
use crate::chunk::block::state::BlockStateType;

pub mod chunk;
pub mod entity;
pub mod rpc;
pub mod spread;
pub mod ty;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_F32: f32 = CHUNK_SIZE as f32;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerBoundWorldPacket {
	RequestChunk(ChunkPos),
	SetBlock(BlockPos, Id<ChunkLayerType>, Id<BlockType>),
	SpawnEntity(Id<EntityType>, Vec<EntityComponentPacket>),
	UpdateEntity(EntityPacket),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientBoundWorldPacket {
	Chunk(ChunkPos, Chunk),
	SetBlock(BlockPos, Id<ChunkLayerType>, Id<BlockType>),
	SpawnEntity(Entity, Id<EntityType>),
	UpdateEntity(EntityPacket),
}

pub struct World {
	pub chunks: ChunkStorage,
	pub entities: EntityWorld,

	spreader: SpreaderSystem,
}

impl World {
	pub fn new(chunk: ChunkStorage) -> Result<World> {
		Ok(World {
			chunks: chunk,
			entities: EntityWorld::new()?,
			spreader: SpreaderSystem::new(),
		})
	}

	pub fn tick(
		&mut self,
		core: &Core,
		rpc: &WorldAPI,
		debug: &mut impl DebugRendererImpl,
	) -> Result<()> {
		for (pos, layer_id, block_id) in self.spreader.tick(rpc, &mut self.chunks, debug) {
			self.place_block(rpc, pos, layer_id, block_id, None)?;
		}
		// Entity
		self.entities
			.tick(core, rpc, &mut self.chunks, debug)
			.wrap_err("Ticking entity")?;
		Ok(())
	}

	pub fn place_block(
		&mut self,
		rpc: &WorldAPI,
		pos: BlockPos,
		layer_id: Id<ChunkLayerType>,
		block_id: Id<BlockType>,
		state: Option<Id<BlockStateType>>
	) -> Result<()> {
		if let Some(chunk) = self.chunks.get_mut(pos.chunk) {
			// Layer
			let layer = &rpc.block_layer[layer_id];
			let desc = layer.blocks.get(block_id.into())?;
			let block = desc.create(layer_id, block_id, state.map(|v| v.into()))?;

			chunk.set_block(pos.entry, block);
			self.spreader
				.place_block(pos, layer_id, block_id, desc);
		}

		Ok(())
	}

	pub fn packet(
		&mut self,
		carrier: &WorldAPI,
		token: Token,
		packet: ServerBoundWorldPacket,
		network: &mut ServerSender<ClientBoundWorldPacket>,
	) -> Result<()> {
		match packet {
			ServerBoundWorldPacket::RequestChunk(chunk_pos) => {
				if let Some(chunk) = self.chunks.get(chunk_pos) {
					network.send(
						token,
						ClientBoundWorldPacket::Chunk(chunk_pos, chunk.clone()),
					)?;
				}
			}
			ServerBoundWorldPacket::SetBlock(pos, layer_id, block_id) => {
				self.place_block(carrier, pos, layer_id, block_id, None)?;
			}
			ServerBoundWorldPacket::SpawnEntity(id, packets) => {
				let entity = self.entities.storage.push(carrier, id);
				network.send(token, ClientBoundWorldPacket::SpawnEntity(entity, id))?;
				for packet in packets {
					let packet = EntityPacket {
						entity,
						component: packet,
					};
					self.entities.packet(&packet);
					network.send(token, ClientBoundWorldPacket::UpdateEntity(packet))?;
				}
			}
			ServerBoundWorldPacket::UpdateEntity(packet) => {
				self.entities.packet(&packet);
				network.send(token, ClientBoundWorldPacket::UpdateEntity(packet))?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn compile() {}
}
