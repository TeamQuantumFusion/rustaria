use anyways::Result;
use apollo::macros::*;

use crate::api::luna::lib::stargate::Stargate;

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
