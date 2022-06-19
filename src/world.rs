use chunk::{block::BlockDesc, layer::BlockLayer};
use eyre::{Result, WrapErr};
use hecs::Entity;

use crate::{
	debug::DebugRendererImpl,
	network::Token,
	packet,
	ty::{block_pos::BlockPos, id::Id},
	world::spread::SpreaderSystem,
	Api, Chunk, ChunkPos, ChunkStorage, EntityWorld, ServerNetwork,
};
use crate::world::entity::prototype::EntityDesc;
use crate::world::entity::system::network::{EntityComponentPacket, EntityPacket};

pub mod chunk;
pub mod entity;
pub mod spread;

packet!(World(ServerBoundWorldPacket, ClientBoundWorldPacket));

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
	pub chunks:   ChunkStorage,
	pub entities: EntityWorld,

	spreader: SpreaderSystem,
}

impl World {
	pub fn new(api: &Api, chunk: ChunkStorage) -> Result<World> {
		Ok(World {
			chunks:   chunk,
			entities: EntityWorld::new(api)?,
			spreader: SpreaderSystem::new(),
		})
	}

	pub fn tick(&mut self, api: &Api, debug: &mut impl DebugRendererImpl) -> eyre::Result<()> {
		for (pos, layer_id, block_id) in self.spreader.tick(api, &mut self.chunks, debug) {
			self.place_block(api, pos, layer_id, block_id);
		}
		// Entity
		self.entities.tick(api, &mut self.chunks, debug).wrap_err("Ticking entity")?;
		Ok(())
	}

	pub fn place_block(
		&mut self,
		api: &Api,
		pos: BlockPos,
		layer_id: Id<BlockLayer>,
		block_id: Id<BlockDesc>,
	) {
		if let Some(chunk) = self.chunks.get_mut(pos.chunk) {
			// Layer
			let layer = chunk.layers.get_mut(layer_id);
			let prototype = api.carrier.block_layer.get(layer_id);

			// Block
			let block_prototype = prototype.blocks.get(block_id);
			layer[pos.entry] = block_prototype.create(block_id);

			self.spreader
				.place_block(pos, layer_id, block_id, block_prototype);
		}
	}

	pub(crate) fn packet(
		&mut self,
		api: &Api,
		token: Token,
		packet: ServerBoundWorldPacket,
		network: &mut ServerNetwork,
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
				self.place_block(api, pos, layer_id, block_id);
			}
			ServerBoundWorldPacket::SpawnEntity(id, packets) => {
				let entity = self.entities.storage.push(api, id);
				network.send(token, ClientBoundWorldPacket::SpawnEntity(entity, id))?;
				for packet in packets {
					let packet = EntityPacket {
						entity,
						component: packet
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
