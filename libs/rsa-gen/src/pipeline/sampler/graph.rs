use crate::pipeline::context::Context;
use crate::pipeline::pass::Pass;
use crate::pipeline::sampler::{BakedSampler, Sampler};
use rustaria_common::ty::Direction;

/// A Graph sampler uses its `sampler` to determine the height of the point to beat.
/// If your position in the direction is higher than the `sampler` value it will be the `more` sampler that will be used,
/// else the `less` sampler will be used.
#[derive(Clone)]
pub struct GraphSampler {
	/// The direction the graph should expand from
	pub(crate) direction: Direction,
	/// The graph node height sampler.
	pub(crate) sampler: Sampler,
	/// If its below the node value.
	pub(crate) less: Sampler,
	/// If its above the node value.
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

	pub fn bake<'a, T: Clone + Default + Send + Sync>(
		&'a self,
		ctx: Context<'a, T>,
		pass: &Pass,
	) -> BakedSampler<'a> {
		let sampler = match self.direction {
			Direction::Right | Direction::Left => self.sampler.bake(
				ctx.clone(),
				&Pass {
					x_range: ctx.max_x()..ctx.max_x() + 1,
					y_range: pass.y_range.clone(),
				},
			),
			Direction::Up | Direction::Down => self.sampler.bake(
				ctx.clone(),
				&Pass {
					x_range: pass.x_range.clone(),
					y_range: ctx.max_y()..ctx.max_y() + 1,
				},
			),
		};

		let less = self.less.bake(ctx.clone(), pass);
		let more = self.more.bake(ctx.clone(), pass);

		BakedSampler::new(move |x, y| {
			let value = match self.direction {
				Direction::Right | Direction::Left => sampler.get(ctx.max_x(), y),
				Direction::Up | Direction::Down => sampler.get(x, ctx.max_y()),
			};

			let pos = ctx.local_dir(x, y, self.direction);

			if value < pos {
				less.get(x, y)
			} else {
				more.get(x, y)
			}
		})
	}
}
