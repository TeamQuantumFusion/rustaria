use std::sync::mpsc::sync_channel;
use futures::executor;
use crate::sweep::sampler::{NoiseKind, Sampler};
use crate::Sweep;
use simdeez::avx2::Avx2;
use simdeez::scalar::Scalar;
use simdeez::sse2::Sse2;
use simdeez::sse41::Sse41;
use simdeez::Simd;
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

	pub fn bake<'a, T: Clone + Default>(
		&'a self,
		sweep: Sweep<'a, T>,
	) -> Box<dyn Fn(u32, u32) -> f32 + 'a> {
		let width = sweep.width();
		let height = sweep.height();
		let min_x = sweep.min_x();
		let min_y = sweep.min_y();


		let values = match self.kind {
			NoiseKind::Simplex => {
				NoiseBuilder::gradient_2d_offset(min_x as f32 + (self.offset * sweep.generator.width as f32),
				                                 width as usize,
				                                 min_y as f32 + (self.offset * sweep.generator.height as f32),
				                                 height as usize).with_freq(
					1.0 / self.scale_x
				).with_seed(sweep.generator.seed as i32).generate_scaled(0.0, 1.0)
			}
		};


		Box::new(move |x, y| {

			values[((x - min_x) + ((y - min_y) * width)) as usize]
		})
	}

	pub fn get<T: Clone + Default>(&self, sweep: &Sweep<T>, x: u32, y: u32) -> f32 {
		match self.kind {
			NoiseKind::Simplex => {
				let x = (x as f32 / self.scale_x) + (self.offset * sweep.generator.width as f32);
				let y = (y as f32 / self.scale_y) + (self.offset * sweep.generator.height as f32);
				(sweep.generator.noiser.simplex.eval_2d(x, y) + 1.0) / 2.0
			}
		}
	}
}

