use crate::pipeline::context::Context;
use crate::pipeline::pass::Pass;
use crate::pipeline::sampler::fade::FadeSampler;
use crate::pipeline::sampler::graph::GraphSampler;
use crate::pipeline::sampler::layer::LayerSampler;
use crate::pipeline::sampler::noise::NoiseSampler;
use crate::pipeline::sampler::split::SplitSampler;
use crate::pipeline::sampler::zoom::ZoomSampler;

pub mod fade;
pub mod graph;
pub mod layer;
pub mod noise;
pub mod split;
pub mod zoom;


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

impl Sampler {
	pub fn bake<'a, T:  Clone + Default + Send + Sync>(&'a self, ctx: Context<'a, T>, pass: &Pass) -> BakedSampler<'a> {
		match self {
			Sampler::X => {
				let width = pass.width() as f32;
				BakedSampler::new(move | x, _| x as f32 / width)
			}
			Sampler::Y => {
				let height = pass.height() as f32;
				BakedSampler::new(move |_, y| y as f32 / height)
			}
			Sampler::Const(value) => {
				let value = *value;
				BakedSampler::new(move |_, _| value)
			}
			Sampler::Layered(sampler) => sampler.bake(ctx, pass),
			Sampler::Zoom(sampler) => sampler.bake(ctx, pass),
			Sampler::Fade(sampler) => sampler.bake(ctx, pass),
			Sampler::Split(sampler) => sampler.bake(ctx, pass),
			Sampler::Graph(sampler) => sampler.bake(ctx, pass),
			Sampler::Noise(sampler) => sampler.bake(ctx, pass),
		}
	}
}


pub struct BakedSampler<'a>(Box<dyn Fn(u32, u32) -> f32 + Sync + Send + 'a>);

impl<'a> BakedSampler<'a> {
	pub fn new<V: 'a + Sync + Send +  Fn(u32, u32) -> f32>(func: V) -> BakedSampler<'a> {
		BakedSampler(Box::new(func))
	}

	pub fn get(&self, x: u32, y: u32) -> f32 {
		self.0(x, y)
	}
}
