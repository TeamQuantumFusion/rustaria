use rustaria_common::ty::Direction;
use crate::pipeline::context::Context;
use crate::pipeline::pass::Pass;
use crate::pipeline::sampler::{BakedSampler, Sampler};

/// Creates a fade between two samplers.
#[derive(Clone)]
pub struct FadeSampler {
	/// What direction it should fade to.
	pub(crate) direction: Direction,
	pub(crate) from: Sampler,
	pub(crate) to: Sampler,
}

impl FadeSampler {
	pub fn new(direction: Direction, from: Sampler, to: Sampler) -> Sampler {
		Sampler::Fade(Box::new(FadeSampler {
			direction,
			from,
			to,
		}))
	}

	pub fn bake<'a, T:  Clone + Default + Send + Sync>(&'a self, ctx: Context<'a, T>, pass: &Pass) -> BakedSampler<'a> {
		let from = self.from.bake(ctx.clone(), pass);
		let to = self.to.bake(ctx.clone(), pass);
		
		BakedSampler::new(move |x, y| {
			let pos = ctx.local_dir(x, y, self.direction);

			let from = from.get(x, y);
			let to = to.get(x, y);
			(from * pos) + (to * (1.0 - pos))
		})
	}
}
