use std::ops::Range;

use rand::Rng;
use fade::FadeSampler;
use graph::GraphSampler;
use noise::NoiseSampler;

use split::SplitSampler;
use zoom::ZoomSampler;
use crate::sweep::sampler::layer::LayerSampler;

use crate::sweep::Sweep;

pub mod zoom;
pub mod fade;
pub mod split;
pub mod graph;
pub mod noise;
pub mod layer;

/// A Sampler returns a value between 0.0..1.0 at a given world coordinate.
/// These may be heavily layered to create advanced and unique value output.
#[derive(Clone)]
pub enum Sampler {
	// Simple
	/// Returns the local X coordinate.
	X,
	/// Returns the local Y coordinate.
	Y,
	/// Returns {0} no matter where on the world the point is sampled.
	Const(f32),

	// Processing
	Layered(LayerSampler),
	Zoom(Box<ZoomSampler>),
	Fade(Box<FadeSampler>),
	Split(Box<SplitSampler>),
	Graph(Box<GraphSampler>),

	// Random
	/// Returns a value between the range using the world generation noise function.
	Random(Range<f32>),
	Noise(NoiseSampler),
}

#[derive(Clone)]
pub enum NoiseKind {
	Simplex,
}

impl Sampler {
	pub fn get<T: Clone + Default>(&self, sweep: &Sweep<T>, x: u32, y: u32) -> f32 {
		match self {
			// Simple values
			Sampler::X => x as f32 / sweep.width() as f32,
			Sampler::Y => y as f32 / sweep.height() as f32,
			Sampler::Const(value) => *value,

			// Processing
			Sampler::Zoom(sampler) => sampler.get(sweep, x, y),
			Sampler::Fade(sampler) => sampler.get(sweep, x, y),
			Sampler::Graph(sampler) => sampler.get(sweep, x, y),
			Sampler::Split(sampler) => sampler.get(sweep, x, y),
			Sampler::Layered(sampler) => sampler.get(sweep, x, y),

			// Noise
			Sampler::Random(range) => sweep.generator.noiser.rng().gen_range(range.clone()),
			Sampler::Noise(sampler) => sampler.get(sweep, x, y),
		}
	}

	pub fn get_sweep<T: Clone + Default>(&self, sweep: &Sweep<T>, x: u32, y: u32) -> f32 {
		match self {
			// Simple values
			Sampler::X => x as f32 / sweep.width() as f32,
			Sampler::Y => y as f32 / sweep.height() as f32,
			Sampler::Const(value) => *value,

			// Processing
			Sampler::Zoom(sampler) => sampler.get(sweep, x, y),
			Sampler::Fade(sampler) => sampler.get(sweep, x, y),
			Sampler::Graph(sampler) => sampler.get(sweep, x, y),
			Sampler::Split(sampler) => sampler.get(sweep, x, y),
			Sampler::Layered(sampler) => sampler.get(sweep, x, y),

			// Noise
			Sampler::Random(range) => sweep.generator.noiser.rng().gen_range(range.clone()),
			Sampler::Noise(sampler) => sampler.get(sweep, x, y),
		}
	}
}
