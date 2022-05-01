use std::ops::Range;
use rustaria_api::ty::Tag;
use crate::Noise;

#[derive(Copy, Clone, Default)]
pub struct BiomeId(pub u16);
#[derive(Copy, Clone, Default)]
pub struct ZoneId(pub u16);
#[derive(Copy, Clone, Default)]
pub struct ClimateId(pub  u16);

pub struct Zone {
	pub w_height: f32,

	pub surface_size: f32,
	pub surface_transition: f32,
	pub surface_biome: BiomeId,
	pub cave_biome: BiomeId,

	// Cache
	pub child_climates: Vec<(ClimateId, Range<u32>)>,
	pub child_biomes: Vec<BiomeId>,


	// Gen values
	pub world_height: Range<u32>
}

pub struct Climate {
	pub shape: ClimateShape,
	pub w_width: f32,

	pub depth: f32,
	pub surface_size: f32,
	pub surface_transition: f32,
	pub surface_biome: BiomeId,
	pub cave_biome: BiomeId,

	// Cache
	pub child_biomes: Vec<BiomeId>,

	// Locations
	pub zones: Vec<ZoneId>
}

pub struct Biome {
	pub color: [u8; 3],
	pub air_ratio: f32,
	pub height_range: Range<f32>,
	pub noise: Noise,

	// Locations
	pub zones: Vec<ZoneId>,
	pub climates: Vec<ClimateId>,
}


pub struct ZoneSettings {
	pub w_height: f32,
	// height priority
	pub priority: f32,

	// size of the surface biome relative to the zone.
	pub surface_size: f32,
	pub surface_transition: f32,
	pub surface_biome: Tag,
	pub cave_biome: Tag,
}

pub struct BiomeSettings {
	pub color: [u8; 3],
	pub air_ratio: f32,
	pub air_scale: f32,
	// relative to the world, NOT THE ZONE
	pub height_range: Range<f32>,

	// Locations
	pub zones: Vec<Tag>,
	pub climates: Vec<Tag>,
}

pub struct ClimateSettings {
	pub shape: ClimateShape,
	pub w_width: f32,

	// The height of the zone * depth is the depth that the climate will go from the surface
	pub depth: f32,
	// size of the surface biome relative to the zone.
	pub surface_size: f32,
	pub surface_transition: f32,
	pub surface_biome: Tag,
	pub cave_biome: Tag,
	// Location
	pub zones: Vec<Tag>
}

#[derive(Debug)]
pub enum ClimateShape {
	Oval { offset_y: f32 },
	Triangle { offset_y: f32 },
	Rectangle { sheer: f32 },
}

impl ClimateShape {
	pub fn inside(&self, x: f32, y: f32) -> bool {
		match self {
			ClimateShape::Oval { offset_y } => {
				let center = [0.5, 0.5 - offset_y];
				let dist_x = center[0] - x;
				let dist_y = center[1] - (y * -(offset_y - 1.0));
				((dist_x * dist_x) + (dist_y * dist_y)).sqrt() < 0.5
			}
			ClimateShape::Triangle { offset_y } => {
				let dist_x = (x - 0.5).abs() * 2.0;
				let dist_y = (1.0 - y) * (1.0 - offset_y);
				dist_y + dist_x < 1.0
			}
			ClimateShape::Rectangle { sheer } => {
				let dist_x = (x - 0.5).abs() * 2.0;
				let dist_y = (1.0 - sheer) + (if x > 0.5 { y } else { 1.0 - y } * sheer);
				dist_y > dist_x
			}
		}
	}
}