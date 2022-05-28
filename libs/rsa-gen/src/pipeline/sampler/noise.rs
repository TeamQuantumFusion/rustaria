use crate::pipeline::context::Context;
use crate::pipeline::pass::Pass;
use crate::pipeline::sampler::{BakedSampler, Sampler};
use simdnoise::NoiseBuilder;

/// Samples a noise map at x and y coordinates. The scaling will affect where those x, y coordinates are sampled.
#[derive(Clone)]
pub struct NoiseSampler {
	/// Scaling for the x coordinate
	pub(crate) scale_x: f32,
	/// Scaling for the y coordinate
	pub(crate) scale_y: f32,
	/// The world offset the noise will be at. This is used to resolve noise conflicts where similar scaled will yield in visually similar results
	pub(crate) offset: f32,
	// currently only simplex is available
	pub(crate) kind: NoiseKind,
	// Noise batch
}

impl NoiseSampler {
	pub fn new(scale: f32, kind: NoiseKind) -> Sampler {
		Sampler::Noise(NoiseSampler {
			scale_x: scale,
			scale_y: scale,
			offset: 0.0,
			kind,
		})
	}

	pub fn new_iso(scale_x: f32, scale_y: f32, kind: NoiseKind) -> Sampler {
		Sampler::Noise(NoiseSampler {
			scale_x,
			scale_y,
			offset: 0.0,
			kind,
		})
	}

	pub fn new_offset(scale: f32, kind: NoiseKind, offset: f32) -> Sampler {
		Sampler::Noise(NoiseSampler {
			scale_x: scale,
			scale_y: scale,
			offset,
			kind,
		})
	}

	pub fn new_iso_offset(scale_x: f32, scale_y: f32, kind: NoiseKind, offset: f32) -> Sampler {
		Sampler::Noise(NoiseSampler {
			scale_x,
			scale_y,
			offset,
			kind,
		})
	}

	pub fn bake<'a, T: Clone + Default + Send + Sync>(
		&'a self,
		ctx: Context<'a, T>,
		pass: &Pass,
	) -> BakedSampler<'a> {
		let width = pass.width();
		let height = pass.height();
		let min_x = pass.min_x();
		let min_y = pass.min_y();

		let values = match self.kind {
			NoiseKind::Simplex => {
				let (mut values, _, _) = NoiseBuilder::gradient_2d_offset(
					min_x as f32 + (self.offset * ctx.generator.width as f32),
					width as usize,
					min_y as f32 + (self.offset * ctx.generator.height as f32),
					height as usize,
				)
				.with_freq(1.0 / self.scale_x)
				.with_seed(ctx.generator.seed as i32)
				.generate();

				for value in &mut values {
					*value = (((*value * 40.0) + 1.0) / 2.0).clamp(0.0, 1.0)
				}

				values
			}
		};

		BakedSampler::new(move |x, y| values[((x - min_x) + ((y - min_y) * width)) as usize])
	}
}

#[derive(Clone)]
pub enum NoiseKind {
	Simplex,
}
