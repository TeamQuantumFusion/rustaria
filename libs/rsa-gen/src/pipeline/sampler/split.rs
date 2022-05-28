use crate::pipeline::context::Context;
use crate::pipeline::pass::Pass;
use crate::pipeline::sampler::{BakedSampler, Sampler};
use rustaria_common::ty::Direction;
use std::ops::Range;

/// Creates a hard edge in a direction between two samplers. Very close to a fade but creates a hard edge.
#[derive(Clone)]
pub struct SplitSampler {
	pub(crate) direction: Direction,
	pub(crate) range: Range<f32>,
	pub(crate) inner: Sampler,
	pub(crate) outer: Sampler,
}

impl SplitSampler {
	pub fn new(direction: Direction, range: Range<f32>, inner: Sampler, outer: Sampler) -> Sampler {
		Sampler::Split(Box::new(SplitSampler {
			direction,
			range,
			inner,
			outer,
		}))
	}

	pub fn bake<'a, T: Clone + Default + Send + Sync>(
		&'a self,
		ctx: Context<'a, T>,
		pass: &Pass,
	) -> BakedSampler<'a> {
		let inner_ctx = if self.direction.vertical() {
			ctx.extend(0.0..1.0, self.range.clone())
		} else {
			ctx.extend(self.range.clone(), 0.0..1.0)
		};

		let inner = self.inner.bake(inner_ctx.clone(), pass);
		let outer = self.outer.bake(inner_ctx, pass);

		BakedSampler::new(move |x, y| {
			let pos = ctx.local_dir(x, y, self.direction);
			if self.range.contains(&(1.0 - pos)) {
				inner.get(x, y)
			} else {
				outer.get(x, y)
			}
		})
		// inner_sweep dropped here
	}
}
