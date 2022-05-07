use std::ops::Range;

use fade::FadeSampler;
use graph::GraphSampler;
use noise::NoiseSampler;
use rand::Rng;

use crate::sweep::sampler::layer::LayerSampler;
use split::SplitSampler;
use zoom::ZoomSampler;

use crate::sweep::Sweep;
use crate::TableMap;

pub mod fade;
pub mod graph;
pub mod layer;
pub mod noise;
pub mod split;
pub mod zoom;

pub type BakedSampler<'a> = Box<dyn Fn(u32, u32) -> f32 + 'a>;

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
	Noise(NoiseSampler),
}

#[derive(Clone)]
pub enum NoiseKind {
	Simplex,
}

impl Sampler {
	pub fn bake<'a, T: Clone + Default>(&'a self, sweep: Sweep<'a, T>) -> BakedSampler<'a> {
		match self {
			Sampler::X => {
				let width = sweep.width() as f32;
				Box::new(move | x, _| x as f32 / width)
			}
			Sampler::Y => {
				let height = sweep.height() as f32;
				Box::new(move |_, y| y as f32 / height)
			}
			Sampler::Const(value) => {
				let value = *value;
				Box::new(move |_, _| value)
			}
			Sampler::Layered(sampler) => sampler.bake(sweep),
			Sampler::Zoom(sampler) => sampler.bake(sweep),
			Sampler::Fade(sampler) => sampler.bake(sweep),
			Sampler::Split(sampler) => sampler.bake(sweep),
			Sampler::Graph(sampler) => sampler.bake(sweep),
			Sampler::Noise(sampler) => sampler.bake(sweep),
		}
	}

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
			Sampler::Noise(sampler) => sampler.get(sweep, x, y),
		}
	}
}
