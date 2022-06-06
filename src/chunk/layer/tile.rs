use serde::{Deserialize, Serialize};
use rsa_core::ty::{KernelIdentifier, Prototype, RawId, Tag};
use std::collections::HashSet;
use rsa_core::api::lua::FromLua;
use crate::api::ty::{ConnectionType, NeighborAware};


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Tile {
	pub id: RawId,
	pub collision: bool,
	pub opaque: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, FromLua)]
pub struct TilePrototype {
	// name: LanguageKey,
	pub sprite: Option<Tag>,
	pub connection: ConnectionType,
	pub collision: bool,
	//   pub opaque: LockableValue<bool>,
	//  #[serde(default = "TilePrototype::default_blast_resistance")]
	//  pub blast_resistance: BlastResistance,
	//   #[serde(default = "TilePrototype::default_break_resistance")]
	//   pub break_resistance: BreakResistance,
}

impl NeighborAware for TilePrototype {
	fn connection_ty(&self) -> ConnectionType {
		self.connection
	}
}

impl KernelIdentifier for Tile {
	fn id(&self) -> RawId {
		self.id
	}
}

impl Prototype for TilePrototype {
	type Item = Tile;

	fn create(&self, id: RawId) -> Tile {
		Tile {
			id,
			// collision: *self.collision.default(),
			// opaque: *self.opaque.default(),
			collision: self.collision,
			opaque: false,
		}
	}

	fn get_sprites(&self, sprites: &mut HashSet<Tag>) {
		if let Some(sprite) = &self.sprite {
			sprites.insert(sprite.clone());
		}
	}

	fn lua_registry_name() -> &'static str {
		"tile"
	}
}
