use crate::plugin::util::Filter;
use crate::registry::{AssetLocation, LanguageKey, Tag};

pub struct TilePrototype {
    id: Tag,
    name: LanguageKey,
    sprite_path: AssetLocation,
    transitional: bool,
    collision: DynamicValue<bool>,
    opaque: DynamicValue<bool>,
    blast_resistance: BlastResistance,
    break_resistance: BreakResistance,
    tile_type: TileType<Tag>
}

pub enum DynamicValue<T> {
    // stored in the global tile
    Fixed,
    // stored per tile
    Dynamic(T)
}

pub enum BlastResistance {
    Some(u32),
    Indestructible
}

pub enum BreakResistance {
    Any,
    Indestructible,
    Axe(u32),
    Pickaxe(u32),
    Hammer(u32)
}

pub enum TileType<T> {
    Default,
    Spreadable {
        spread_chance: f32,
        filter: Filter<T>
    }
}
