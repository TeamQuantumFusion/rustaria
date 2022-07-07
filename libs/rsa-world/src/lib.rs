//! # Rustaria World
//! Contains code for the world itself and all of its contents. Stuff like entities and blocks are here.
use hecs::Entity;
use rpc::WorldRPC;
use rsa_core::{
	api::Core,
	debug::DebugRendererImpl,
	err::{ext::AuditExt, Result},
	ty::{Id, Registry},
};
use rsa_network::{packet::PacketSetup, server::ServerSender, Token};

use crate::{
	chunk::{block::BlockDesc, layer::BlockLayer, storage::ChunkStorage, Chunk},
	entity::{
		prototype::EntityDesc,
		system::network::{EntityComponentPacket, EntityPacket},
		EntityWorld,
	},
	spread::SpreaderSystem,
	ty::{BlockPos, ChunkPos},
};

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
	SetBlock(BlockPos, Id<BlockLayer>, Id<BlockDesc>),
	SpawnEntity(Id<EntityDesc>, Vec<EntityComponentPacket>),
	UpdateEntity(EntityPacket),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientBoundWorldPacket {
	Chunk(ChunkPos, Chunk),
	SetBlock(BlockPos, Id<BlockLayer>, Id<BlockDesc>),
	SpawnEntity(Entity, Id<EntityDesc>),
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
		rpc: &WorldRPC,
		debug: &mut impl DebugRendererImpl,
	) -> Result<()> {
		for (pos, layer_id, block_id) in self.spreader.tick(rpc, &mut self.chunks, debug) {
			self.place_block(rpc, pos, layer_id, block_id);
		}
		// Entity
		self.entities
			.tick(core, rpc, &mut self.chunks, debug)
			.wrap_err("Ticking entity")?;
		Ok(())
	}

	pub fn place_block(
		&mut self,
		rpc: &WorldRPC,
		pos: BlockPos,
		layer_id: Id<BlockLayer>,
		block_id: Id<BlockDesc>,
	) {
		if let Some(chunk) = self.chunks.get_mut(pos.chunk) {
			// Layer
			let layer = chunk.layers.get_mut(layer_id);
			let prototype = rpc.block_layer.get(layer_id);

			// Block
			let block_prototype = prototype.blocks.get(block_id);
			layer[pos.entry] = block_prototype.create(block_id);

			self.spreader
				.place_block(pos, layer_id, block_id, block_prototype);
		}
	}

	pub fn packet(
		&mut self,
		carrier: &WorldRPC,
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
				self.place_block(carrier, pos, layer_id, block_id);
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
