use crate::Sweep;
use crate::sweep::sampler::{BakedSampler, Sampler};

/// Merges multiple samplers together. Supports weighing.
#[derive(Clone)]
pub struct LayerSampler {
	layers: Vec<(f32, Sampler)>
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

		Sampler::Layered(LayerSampler {
			layers
		})
	}

	pub fn bake<'a, T: Clone + Default>(&'a self, sweep: Sweep<'a, T>) -> BakedSampler<'a> {
		let mut functions = Vec::new();
		for (bias, sampler) in &self.layers {
			functions.push((*bias, sampler.bake(sweep.clone())));
		}

		Box::new(move |x, y| {
			let mut out = 0.0;
			for (bias, func) in &functions {
				out += func(x, y) * *bias;
			}
			out
		})
	}

	pub fn get<T: Clone + Default>(&self, sweep: &Sweep<T>, x: u32, y: u32) -> f32 {
		let mut out = 0.0;
		for (bias, sampler) in &self.layers {
			out += sampler.get(sweep, x, y) * bias;
		}
		out
	}
}