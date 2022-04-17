use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use rustaria_api::ty::{Prototype, RawId, Tag};

use crate::{api::ty::ConnectionType, chunk::Tile};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TilePrototype {
	// name: LanguageKey,
	pub sprite: Option<Tag>,
	pub connection: ConnectionType,
	// #[serde(default = "TilePrototype::default_collision")]
	//  pub collision: LockableValue<bool>,
	//  #[serde(default = "TilePrototype::default_opaque")]
	//   pub opaque: LockableValue<bool>,
	//  #[serde(default = "TilePrototype::default_blast_resistance")]
	//  pub blast_resistance: BlastResistance,
	//   #[serde(default = "TilePrototype::default_break_resistance")]
	//   pub break_resistance: BreakResistance,
}

impl Prototype for TilePrototype {
	type Item = Tile;

	fn create(&self, id: RawId) -> Tile {
		Tile {
			id,
			// collision: *self.collision.default(),
			// opaque: *self.opaque.default(),
			collision: false,
			opaque: false,
		}
	}

	fn get_sprites(&self, sprites: &mut HashSet<Tag>) {
		if let Some(sprite) = &self.sprite {
			sprites.insert(sprite.clone());
		}
	}

	fn lua_registry_name() -> &'static str {
		"Tiles"
	}
}
