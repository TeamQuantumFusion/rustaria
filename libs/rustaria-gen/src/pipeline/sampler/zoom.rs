use std::ops::Range;
use crate::pipeline::context::Context;
use crate::pipeline::pass::Pass;
use crate::pipeline::sampler::{BakedSampler, Sampler};

#[derive(Clone)]
/// Extends the inner samplers range.
/// - anything below `start` will be 0,
/// - anything above `end` will be 1,
/// - anything between `start`..`end` will be scaled to those values. so a inner of 0.5 will give the value that is between `start` and `end`
pub struct ZoomSampler {
	pub(crate) range: Range<f32>,
	pub(crate) sampler: Sampler,
}

impl ZoomSampler {
	pub fn new(range: Range<f32>, sampler: Sampler) -> Box<ZoomSampler> {
		Box::new(ZoomSampler { range, sampler })
	}

	pub fn bake<'a, T:  Clone + Default + Send + Sync>(&'a self, ctx: Context<'a, T>, pass: &Pass) -> BakedSampler<'a> {
		let sampler = self.sampler.bake(ctx, pass);
		let range = self.range.clone();

		BakedSampler::new(move |x, y| {
			let low = (sampler.get(x, y) - range.start).max(0.0);
			(low / (range.end - range.start)).clamp(0.0, 1.0)
		})
	}
}