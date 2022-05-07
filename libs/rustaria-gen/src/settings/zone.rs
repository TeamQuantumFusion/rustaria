use crate::settings::BiomeProducerSettings;

pub struct ZoneSettings {
	pub w_height: f32,
	// height priority
	pub priority: f32,

	pub terrain_size: f32,
	pub biome_producer: BiomeProducerSettings,
}
