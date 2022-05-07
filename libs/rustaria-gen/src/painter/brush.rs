use crate::util::pass::WorldPass;
use crate::{Generator, Noise, TableMap};
use crate::sweep::sampler::Sampler;
use crate::sweep::Sweep;

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
	Layered {
		layers: Vec<Brush<T>>
	}
}

impl<T: Clone + Default> Brush<T> {
	pub fn apply(
		&self,
		sweep: &Sweep<T>,
		x: u32,
		y: u32,
		map: &mut TableMap<T>,
	) {
		match self {
			Brush::Fill(value) => {
				map.insert(x, y, value.clone());
			},
			Brush::Selector { sampler, values } => {
				let value = sampler.get(sweep, x, y);
				for (threshold, brush) in values {
					if *threshold > value {
						return brush.apply(sweep, x, y, map);
					}
				}
			}
			Brush::Layered { layers } => {
				for brush in layers {
					brush.apply(sweep,  x, y, map);
				}
			}
			_ => {}
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
		Brush::Layered {
			layers
		}
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
