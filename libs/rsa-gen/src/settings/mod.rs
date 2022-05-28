use crate::{BiomeId, BiomeSettings, ClimateId, ClimateSettings, ZoneId, ZoneSettings};
use rustaria_common::ty::Tag;
use std::collections::HashMap;

pub mod biome;
pub mod climate;
pub mod zone;

pub struct GenerationSettings<T: Clone> {
	pub zones: Vec<(Tag, ZoneSettings)>,
	pub climates: Vec<(Tag, ClimateSettings)>,
	pub biomes: Vec<(Tag, BiomeSettings<T>)>,
	// [0..1] of the world which is spawn protected
	pub spawn_size: f32,
	// [0..1] of zone height
	pub biome_height_transition: f32,

	pub seed: u32,
	// tiles width*height
	pub width: u32,
	pub height: u32,
}
impl<T: Clone> GenerationSettings<T> {
	pub fn sort(&mut self) {
		self.zones
			.sort_by(|(_, v0), (_, v1)| v0.priority.total_cmp(&v1.priority));
		self.climates.sort_by(|(v0, _), (v1, _)| v0.cmp(v1));
		self.biomes.sort_by(|(v0, _), (v1, _)| v0.cmp(v1));
	}

	pub fn zone_lookup(&self) -> HashMap<Tag, ZoneId> {
		self.zones
			.iter()
			.enumerate()
			.map(|(id, (tag, _))| (tag.clone(), ZoneId(id as u16)))
			.collect()
	}

	pub fn climate_lookup(&self) -> HashMap<Tag, ClimateId> {
		self.climates
			.iter()
			.enumerate()
			.map(|(id, (tag, _))| (tag.clone(), ClimateId(id as u16)))
			.collect()
	}
	pub fn biome_lookup(&self) -> HashMap<Tag, BiomeId> {
		self.biomes
			.iter()
			.enumerate()
			.map(|(id, (tag, _))| (tag.clone(), BiomeId(id as u16)))
			.collect()
	}
}

/// A biome producer explains the intended biome generation across the area (zone or climate)
pub struct BiomeProducerSettings {
	/// The fraction on how much of the area will be dedicated to the surface biome
	pub surface_size: f32,
	/// The fraction on how much of the area will be dedicated to the biome transition between surface and cave.
	pub surface_transition: f32,
	/// The surface biome type.
	pub surface_biome: Tag,
	/// The cave biome type.
	pub cave_biome: Tag,
}

pub struct BiomeProducer {
	pub surface_size: f32,
	pub surface_transition: f32,
	pub surface_biome: BiomeId,
	pub cave_biome: BiomeId,
}

impl BiomeProducer {
	pub fn new(
		settings: BiomeProducerSettings,
		biomes_lookup: &HashMap<Tag, BiomeId>,
	) -> BiomeProducer {
		BiomeProducer {
			surface_size: settings.surface_size,
			surface_transition: settings.surface_transition,
			surface_biome: *biomes_lookup.get(&settings.surface_biome).unwrap(),
			cave_biome: *biomes_lookup.get(&settings.cave_biome).unwrap(),
		}
	}
}
