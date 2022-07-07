use anyways::Result;
use apollo::macros::*;

use crate::{
	api::{stargate::Stargate, Core},
	blake3::Hasher,
};

pub struct Reload {
	pub stargate: Stargate,
	pub client: bool,
}

impl Reload {}

#[lua_impl]
impl Reload {
	#[lua_field(get stargate)]
	pub fn get_stargate(&mut self) -> Result<&mut Stargate> { Ok(&mut self.stargate) }

	#[lua_field(get client)]
	pub fn get_client(&mut self) -> Result<bool> { Ok(self.client) }
}

pub trait RustariaPrototypeCarrier {
	fn register_registries(&mut self, core: &mut Core, reload: &mut Reload) -> Result<()>;
	fn build_registries(&mut self, core: &mut Core, reload: &mut Reload) -> Result<()>;
	fn append_hasher(&mut self, hasher: &mut Hasher) -> Result<()>;
}
