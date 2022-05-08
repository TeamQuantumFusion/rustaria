use std::collections::HashMap;
use std::ops::Range;
use rustaria_api::ty::Tag;
use crate::{ClimateSettings, ZoneSettings};
use crate::pipeline::brush::Brush;
use crate::settings::BiomeProducer;
use crate::pipeline::sampler::Sampler;

#[derive(Copy, Clone, Default)]
pub struct BiomeId(pub u16);
#[derive(Copy, Clone, Default)]
pub struct ZoneId(pub u16);
#[derive(Copy, Clone, Default)]
pub struct ClimateId(pub  u16);

pub struct Zone {
	pub w_height: f32,


	pub terrain_size: f32,
	pub biome_producer: BiomeProducer,

	// Cache
	pub child_climates: Vec<(ClimateId, Range<u32>)>,
	pub child_biomes: Vec<BiomeId>,

	// Gen values
	pub world_range: Range<u32>
}

impl Zone {
	pub fn new(settings: ZoneSettings, biomes_lookup: &HashMap<Tag, BiomeId>) -> Zone {
		Zone {
			w_height: settings.w_height,
			terrain_size: settings.terrain_size,
			biome_producer: BiomeProducer::new(settings.biome_producer, biomes_lookup),
			child_climates: vec![],
			child_biomes: vec![],
			world_range: 0..0,
		}
	}
}

pub struct Climate {
	pub shape: ClimateShape,
	pub w_width: f32,

	pub terrain_size: f32,
	pub depth: f32,

	pub biome_producer: BiomeProducer,

	// Cache
	pub child_biomes: Vec<BiomeId>,

	// Locations
	pub zones: Vec<ZoneId>
}
impl Climate {
	pub fn new(settings: ClimateSettings, zones_out: &mut[Zone], zones_lookup: &HashMap<Tag, ZoneId>, biomes_lookup: &HashMap<Tag, BiomeId>, pos: u16) -> Climate  {
		Climate {
			shape: settings.shape,
			w_width: settings.w_width,
			terrain_size: settings.terrain_size,
			depth: settings.depth,
			biome_producer: BiomeProducer::new(settings.biome_producer, biomes_lookup),
			child_biomes: vec![],
			zones: settings.zones.iter().map(|zone| {
				let x = *zones_lookup.get(zone).unwrap();
				zones_out[x.0 as usize].child_climates.push((ClimateId(pos), 0..0));
				x
			}).collect()
		}
	}
}


pub struct Biome<T: Clone> {
	pub color: [u8; 3],
	pub biome_ratio: f32,
	pub height_range: Range<f32>,
	pub selection_sampler: Sampler,

	pub painter: Brush<T>,

	// Locations
	pub zones: Vec<ZoneId>,
	pub climates: Vec<ClimateId>,
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