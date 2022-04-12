use std::collections::HashSet;

use serde::Deserialize;
use rustaria_api::lua_runtime::UserData;

use rustaria_api::prototype::Prototype;
use rustaria_api::RawId;
use rustaria_api::tag::Tag;

use crate::api::ty::{BlastResistance, BreakResistance, ConnectionType, LockableValue, TileType};
use crate::world::tile::Tile;

#[derive(Clone, Debug, Deserialize)]
pub struct TilePrototype {
    // name: LanguageKey,
    #[serde(default)]
    #[cfg(feature = "client")]
    pub sprite: Option<Tag>,
    #[serde(default = "TilePrototype::default_connection")]
    pub connection: ConnectionType,
    #[serde(default = "TilePrototype::default_collision")]
    pub collision: LockableValue<bool>,
    #[serde(default = "TilePrototype::default_opaque")]
    pub opaque: LockableValue<bool>,
    #[serde(default = "TilePrototype::default_blast_resistance")]
    pub blast_resistance: BlastResistance,
    #[serde(default = "TilePrototype::default_break_resistance")]
    pub break_resistance: BreakResistance,
    #[serde(default)]
    pub tile_type: TileType<Tag>,
}

impl TilePrototype {
    fn default_connection() -> ConnectionType {
        ConnectionType::Connected
    }
    fn default_collision() -> LockableValue<bool> {
        LockableValue::Dynamic(true)
    }
    fn default_opaque() -> LockableValue<bool> {
        LockableValue::Dynamic(true)
    }
    fn default_blast_resistance() -> BlastResistance {
        BlastResistance::Some(3)
    }
    fn default_break_resistance() -> BreakResistance {
        BreakResistance::Any
    }
}
impl Prototype for TilePrototype {
    type Item = Tile;

    fn create(&self, id: RawId) -> Tile {
        Tile {
            id,
            collision: *self.collision.default(),
            opaque: *self.opaque.default(),
        }
    }
 
    fn get_sprites(&self, sprites: &mut HashSet<Tag>) {
        if let Some(sprite) = &self.sprite {
            sprites.insert(sprite.clone());
        }
    }
    
    fn name() -> &'static str {
        "tile"
    }
}
impl UserData for TilePrototype {}
