use std::ops::Range;
use rustaria_api::ty::Tag;
use crate::painter::brush::Brush;

// docs
#[allow(unused_imports)]
use crate::noise::Noise;
use crate::sweep::sampler::Sampler;

pub struct BiomeSettings<T: Clone> {
	// Debugging
	/// This label is used for visualization purposes on the biome map.
	pub label: [u8; 3],

	// Placement
	/// The higher the value the more of the biome will be generated on the map.
	/// Its a noise map that gets sampled. And if its below air_ratio it will make that land this biome.
	pub biome_ratio: f32,
	/// This is the noise that will be used to sample if the biome should be placed.
	pub selection_sampler: Sampler,
	/// This is the height range of the zone that you can spawn in. A value of 0.5..1.0 means you can spawn half way down the zone and down.
	pub height_range: Range<f32>,

	// Locations
	/// The zones you want to spawn in.
	/// This is generated below climates so if you dont explicitly add yourself to the surface climates you wont be present in those.
	pub zones: Vec<Tag>,
	/// The climates you want to spawn in.
	pub climates: Vec<Tag>,

	// Painting
	/// Used for painting the ground area.
	/// # Warning
	/// You probably dont want to generate noise like caves.
	/// To get your noise to more ground generation levels, you can use a [Noise::FillGradient] downwards
	/// to make a gradient down which will more and more fix the value to 1.
	pub ground_painter: Brush<T>,
	pub cave_painter: Brush<T>,
}