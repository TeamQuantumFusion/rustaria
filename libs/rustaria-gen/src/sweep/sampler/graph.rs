use rustaria_common::ty::Direction;
use crate::Sweep;
use crate::sweep::sampler::Sampler;

/// A Graph sampler uses its `sampler` to determine the height of the point to beat.
/// If your position in the direction is higher than the `sampler` value it will be the `more` sampler that will be used,
/// else the `less` sampler will be used.
#[derive(Clone)]
pub struct GraphSampler {
	pub(crate) direction: Direction,
	pub(crate) sampler: Sampler,
	pub(crate) less: Sampler,
	pub(crate) more: Sampler,
}

impl GraphSampler {
	pub fn new(
		direction: Direction,
		stitch_sampler: Sampler,
		less: Sampler,
		more: Sampler,
	) -> Sampler {
		Sampler::Graph(Box::new(GraphSampler {
			direction,
			sampler: stitch_sampler,
			less,
			more,
		}))
	}

	pub fn get<T: Clone + Default>(&self, sweep: &Sweep<T>, x: u32, y: u32) -> f32 {
		let value = match self.direction {
			Direction::Left => self.sampler.get(sweep, sweep.max_x(), y),
			Direction::Right => self.sampler.get(sweep, sweep.max_x(), y),
			Direction::Up => self.sampler.get(sweep, x, sweep.min_y()),
			Direction::Down => self.sampler.get(sweep, x, sweep.min_y()),
		};

		let pos = sweep.local_dir(x, y, self.direction);


		if value < pos {
			self.less.get(sweep, x, y)
		} else {
			self.more.get(sweep, x, y)
		}
	}
}
