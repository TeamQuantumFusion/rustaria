use std::{collections::HashSet, mem::replace, ops::Deref, path::PathBuf};

use apollo::Lua;
use rsa_client_core::{atlas::Atlas, frontend::Frontend};
use rsa_client_graphics::{
	world::{chunk::layer::BlockLayerRendererPrototype, entity::EntityRendererPrototype},
	GraphicsRPC,
};
use rsa_core::{
	api::{
		reload::{Reload, RustariaPrototypeCarrier},
		stargate::Stargate,
		Core,
	},
	blake3::Hasher,
	err::Result,
};
use rustaria::rpc::ServerRPC;

use crate::AuditExt;

#[derive(Default)]
pub struct ClientRPC {
	pub server: ServerRPC,
	pub graphics: GraphicsRPC,
}

impl ClientRPC {
	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		ServerRPC::register(stargate, lua)?;
		GraphicsRPC::register(stargate, lua)?;
		Ok(())
	}

	pub fn build(frontend: &Frontend, core: &Core, stargate: &mut Stargate) -> Result<ClientRPC> {
		let server = ServerRPC::build(stargate).wrap_err("Failed to build ServerRPC")?;
		let graphics = GraphicsRPC::build(&frontend, &server, core, stargate)
			.wrap_err("Failed to build GraphicsRPC")?;
		Ok(ClientRPC { graphics, server })
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) { self.server.append_hasher(hasher); }
}

impl Deref for ClientRPC {
	type Target = ServerRPC;

	fn deref(&self) -> &Self::Target { &self.server }
}
