use rustaria_common::ty::Direction;
use std::ops::Range;
use crate::Sweep;
use crate::sweep::sampler::Sampler;

/// Creates a hard edge in a direction between two samplers. Very close to a fade but creates a hard edge.
#[derive(Clone)]
pub struct SplitSampler {
	pub(crate) direction: Direction,
	pub(crate) range: Range<f32>,
	pub(crate) inner: Sampler,
	pub(crate) outer: Sampler,
}

impl SplitSampler {
	pub fn new(direction: Direction,range: Range<f32>,  inner: Sampler, outer: Sampler) -> Sampler {
		Sampler::Split(Box::new(SplitSampler {
			direction,
			range,
			inner,
			outer
		}))
	}

	pub fn get<T: Clone + Default>(&self, sweep: &Sweep<T>, x: u32, y: u32) -> f32 {
		let pos = sweep.local_dir(x, y, self.direction);

		let inner_sweep = if self.direction.vertical() {
			sweep.extend(0.0..1.0, self.range.clone())
		} else {
			sweep.extend(self.range.clone(), 0.0..1.0)
		};
		if self.range.contains(&(1.0 - pos)) {
			self.inner.get(&inner_sweep, x, y)
		} else {
			self.outer.get(&inner_sweep, x, y)
		}
	}
}
