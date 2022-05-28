use crate::pipeline::context::Context;
use crate::pipeline::pass::Pass;
use crate::pipeline::sampler::{BakedSampler, Sampler};

/// Merges multiple samplers together. Supports weighing.
#[derive(Clone)]
pub struct LayerSampler {
	layers: Vec<(f32, Sampler)>,
}

impl LayerSampler {
	pub fn new(layers: Vec<Sampler>) -> Sampler {
		Self::new_weighted(layers.into_iter().map(|sampler| (1.0, sampler)).collect())
	}

	pub fn new_weighted(mut layers: Vec<(f32, Sampler)>) -> Sampler {
		let mut total = 0.0;
		for (weight, _) in &layers {
			total += *weight;
		}

		for (weight, _) in &mut layers {
			*weight /= total;
		}

		Sampler::Layered(LayerSampler { layers })
	}

	pub fn bake<'a, T: Clone + Default + Send + Sync>(
		&'a self,
		ctx: Context<'a, T>,
		pass: &Pass,
	) -> BakedSampler<'a> {
		let mut functions = Vec::new();
		for (bias, sampler) in &self.layers {
			functions.push((*bias, sampler.bake(ctx.clone(), pass)));
		}

		BakedSampler::new(move |x, y| {
			let mut out = 0.0;
			for (bias, func) in &functions {
				out += func.get(x, y) * *bias;
			}
			out
		})
	}
}
