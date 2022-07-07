use std::{collections::HashSet, mem::replace, ops::Deref, path::PathBuf};
use apollo::Lua;
use rsa_core::api::Core;
use rsa_core::api::reload::{Reload, RustariaPrototypeCarrier};
use rsa_core::api::stargate::Stargate;
use rsa_core::blake3::Hasher;
use rsaclient_graphics::GraphicsRPC;
use rustaria::rpc::ServerRPC;
use rsa_core::err::Result;
use rsaclient_core::atlas::Atlas;
use rsaclient_core::frontend::Frontend;
use rsaclient_graphics::world::chunk::layer::BlockLayerRendererPrototype;
use rsaclient_graphics::world::entity::EntityRendererPrototype;
use crate::AuditExt;

#[derive(Default)]
pub struct ClientRPC {
	pub server: ServerRPC,
	pub graphics: GraphicsRPC
}

impl ClientRPC {
	pub fn register(stargate: &mut Stargate, lua: &Lua) -> Result<()> {
		ServerRPC::register(stargate, lua)?;
		GraphicsRPC::register(stargate, lua)?;
		Ok(())
	}

	pub fn build(frontend: &Frontend, core: &Core, stargate: &mut Stargate) -> Result<ClientRPC> {
		let server = ServerRPC::build(stargate).wrap_err("Failed to build ServerRPC")?;
		let graphics = GraphicsRPC::build(&frontend, &server, core, stargate).wrap_err("Failed to build GraphicsRPC")?;
		Ok(ClientRPC {
			graphics,
			server,
		})
	}

	pub fn append_hasher(&mut self, hasher: &mut Hasher) {
		self.server.append_hasher(hasher);
	}
}

impl Deref for ClientRPC {
	type Target = ServerRPC;

	fn deref(&self) -> &Self::Target {
		&self.server
	}
}