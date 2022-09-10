use apollo::Lua;
use rsa_core::{
	api::{
		stargate::Stargate,
	},
	err::{ext::AuditExt, Result},
};
use rsa_hash::Hasher;
use rsa_item::ItemRPC;
use rsa_world::rpc::WorldRPC;

#[derive(Default)]
pub struct ServerRPC {
	pub world: WorldRPC,
	pub item: ItemRPC,
}

impl ServerRPC {
	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		WorldRPC::register(stargate, lua)?;
		ItemRPC::register(stargate, lua)?;
		Ok(())
	}

	pub fn build(stargate: &mut Stargate) -> Result<ServerRPC> {
		Ok(ServerRPC {
			world: WorldRPC::build(stargate).wrap_err("Failed to build WorldRPC")?,
			item: ItemRPC::build(stargate).wrap_err("Failed to build ItemRPC")?,
		})
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) {
		self.world.append_hasher(hasher);
		self.item.append_hasher(hasher);
	}
}
