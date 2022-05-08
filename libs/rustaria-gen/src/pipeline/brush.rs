use crate::pipeline::context::Context;
use crate::pipeline::pass::Pass;
use crate::pipeline::sampler::Sampler;

// Places stuff down
#[derive(Clone)]
pub enum Brush<T: Clone> {
	/// Overwrites the current position with this value.
	Fill(T),
	/// Ignores the value.
	Ignore,
	/// Samples from the selector and returns the value as the index.
	Selector {
		sampler: Sampler,
		values: Vec<(f32, Brush<T>)>,
	},
	/// Runs all of these layers on top of each other
	Layered { layers: Vec<Brush<T>> },
}

impl<T: Clone + Default + Send + Sync> Brush<T> {
	pub fn bake<'a>(&'a self, ctx: Context<'a, T>, pass: &Pass) -> BakedBrush<'a, T> {
		match self {
			Brush::Fill(value) => BakedBrush::new(|_, _, v| *v = value.clone()),
			Brush::Selector { sampler, values } => {
				let sampler = sampler.bake(ctx.clone(), pass);
				let mut out = Vec::new();
				for (threshold, brush) in values {
					out.push((*threshold, brush.bake(ctx.clone(), pass)))
				}

				BakedBrush::new(move |x, y, v| {
					let value: f32 = sampler.get(x, y);
					for (threshold, brush) in &out {
						if *threshold > value {
							return brush.apply(x, y, v);
						}
					}

				})
			}
			Brush::Layered { layers } => {
				let mut out = Vec::new();
				for brush in layers {
					out.push(brush.bake(ctx.clone(), pass))
				}

				BakedBrush::new(move |x, y, v| {
					for brush in &out {
						brush.apply(x, y, v)
					}
				})
			}
			_ => BakedBrush::new(|_, _, _| ()),
		}
	}
}

impl<T: Clone> Brush<T> {
	pub fn set(value: T) -> Brush<T> {
		Brush::Fill(value)
	}

	pub fn none() -> Brush<T> {
		Brush::Ignore
	}

	pub fn layered(layers: Vec<Brush<T>>) -> Brush<T> {
		Brush::Layered { layers }
	}

	pub fn noise(noise: Sampler, values: Vec<Brush<T>>) -> Brush<T> {
		Self::noise_weighted(noise, values.into_iter().map(|v| (1.0, v)).collect())
	}

	pub fn noise_weighted(sampler: Sampler, mut values: Vec<(f32, Brush<T>)>) -> Brush<T> {
		let mut total_weight = 0.0;
		for (weight, _) in &values {
			total_weight += weight;
		}

		let mut current_weight = 0.0;
		for (weight, _) in &mut values {
			let corrected_weight = *weight / total_weight;
			current_weight += corrected_weight;
			*weight = current_weight;
		}

		Brush::Selector { sampler, values }
	}
}

pub struct BakedBrush<'a, T>(Box<dyn Fn(u32, u32, &mut T) + 'a + Send + Sync>);

impl<'a, T> BakedBrush<'a, T> {
	pub fn new<F: Fn(u32, u32, &mut T) + 'a + Send + Sync>(func: F) -> Self {
		BakedBrush(Box::new(func))
	}

	pub fn apply(&self, x: u32, y: u32, value: &mut T) {
	self.0(x, y, value)
	}

}
