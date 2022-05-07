use crate::sweep::sampler::Sampler;
use crate::Sweep;
use rustaria_common::ty::Direction;

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

	pub fn get<T: Clone + Default>(&self, sweep: &Sweep<T>, x: u32, y: u32) -> f32 {
		let pos = sweep.local_dir(x, y, self.direction);

		let from = self.from.get(sweep, x, y);
		let to = self.to.get(sweep, x, y);
		(from * pos) + (to * (1.0 - pos))
	}
}
