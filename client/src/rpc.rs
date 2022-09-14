use std::{ops::Deref};

use apollo::Lua;
use rsa_client_core::{frontend::Frontend};
use rsa_client_graphics::{
	GraphicsRPC,
};
use rsa_core::{
	api::{
		stargate::Stargate,
		Core,
	},
	err::Result,
};
use rsa_hash::Hasher;
use rustaria::api::RustariaAPI;

use crate::AuditExt;

#[derive(Default)]
pub struct ClientRPC {
	pub server: RustariaAPI,
	pub graphics: GraphicsRPC,
}

impl ClientRPC {
	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		RustariaAPI::register(stargate, lua)?;
		GraphicsRPC::register(stargate, lua)?;
		Ok(())
	}

	pub fn build(frontend: &Frontend, core: &Core, stargate: &mut Stargate) -> Result<ClientRPC> {
		let server = RustariaAPI::build(stargate).wrap_err("Failed to build ServerRPC")?;
		let graphics = GraphicsRPC::build(&frontend, &server, core, stargate)
			.wrap_err("Failed to build GraphicsRPC")?;
		Ok(ClientRPC { graphics, server })
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) { self.server.append_hasher(hasher); }
}

impl Deref for ClientRPC {
	type Target = RustariaAPI;

	fn deref(&self) -> &Self::Target { &self.server }
}