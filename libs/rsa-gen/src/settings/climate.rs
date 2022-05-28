use crate::settings::BiomeProducerSettings;
use crate::ty::ClimateShape;
use rustaria_common::ty::Tag;

pub struct ClimateSettings {
	pub shape: ClimateShape,
	pub w_width: f32,

	pub terrain_size: f32,
	// The height of the zone * depth is the depth that the climate will go from the surface
	pub depth: f32,
	pub biome_producer: BiomeProducerSettings,
	// Location
	pub zones: Vec<Tag>,
}
