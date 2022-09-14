use apollo::{macros::*, Lua};
use rsa_core::{
	api::stargate::Stargate,
	err::{ext::AuditExt, Result},
};
use rsa_hash::Hasher;
use rsa_item::ItemAPI;
use rsa_world::{
	rpc::WorldAPI,
	ty::{BlockLayerPos, BlockPos, ChunkPos},
};

#[derive(Default)]
pub struct RustariaAPI {
	pub world: WorldAPI,
	pub item: ItemAPI,
	pub util: UtilAPI,
}

#[lua_impl]
impl RustariaAPI {
	#[lua_field(get world)]
	pub fn world(&self) -> &WorldAPI { &self.world }

	#[lua_field(get item)]
	pub fn item(&self) -> &ItemAPI { &self.item }

	#[lua_field(get util)]
	pub fn util(&self) -> &UtilAPI { &self.util }

	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		WorldAPI::register(stargate, lua)?;
		ItemAPI::register(stargate, lua)?;
		Ok(())
	}

	pub fn build(stargate: &mut Stargate) -> Result<RustariaAPI> {
		Ok(RustariaAPI {
			world: WorldAPI::build(stargate).wrap_err("Failed to build WorldRPC")?,
			item: ItemAPI::build(stargate).wrap_err("Failed to build ItemRPC")?,
			util: Default::default(),
		})
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) {
		self.world.append_hasher(hasher);
		self.item.append_hasher(hasher);
	}
}

#[derive(Default)]
pub struct UtilAPI;

#[lua_impl]
impl UtilAPI {
	#[lua_method]
	pub fn new_block_pos(x: i64, y: i64) -> Option<BlockPos> { 
		BlockPos::try_from((x, y)).ok()
	}

	#[lua_method]
	pub fn new_chunk_pos(x: i64, y: i64) -> Option<ChunkPos> { 
		ChunkPos::try_from((x, y)).ok()
	}

	#[lua_method]
	pub fn new_block_layer_pos(x: u8, y: u8) -> Option<BlockLayerPos> {
		BlockLayerPos::try_new(x, y)
	}
}
