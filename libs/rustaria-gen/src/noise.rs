use std::mem::transmute;
use std::ops::Range;

use opensimplex_noise_rs::OpenSimplexNoise;

pub enum Noise {
	Sample(Box<SampleNoise>),
	Layered(Vec<(Noise, f32)>),
}

impl Noise {
	pub fn new(seed: u64, scale: f32) -> Noise {
		Noise::Sample(Box::new(SampleNoise::new(seed, scale)))
	}

	pub fn octave(seed: u64, scale: f32, octaves: u8) -> Noise{
		let mut layers = Vec::new();
		for i in 0..octaves {
			layers.push((Self::new(seed + i as u64, scale / (1 << i) as f32), 1.0 / octaves as f32));
		}
		Noise::Layered(layers)
	}


	pub fn biome_sample(&self, x: f32, y: f32, height_range: Range<f32>, transition: f32) -> f32 {
		let bias = if height_range.contains(&y) {
			1.0
		} else {
			let height = height_range.end - height_range.start;
			let middle = height_range.start + (height / 2.0);
			let middle_distance = (y - middle).abs();
			let distance = (middle_distance - (height / 2.0)).max(0.0);
			1.0 - (distance.clamp(0.0, transition) / transition)
		};

		self.sample(x, y) * bias
	}

	pub fn sample(&self, x: f32, y: f32) -> f32 {
		match self {
			Noise::Sample(noise) => noise.sample(x, y),
			Noise::Layered(layers) => {
				let mut total = 0.0;
				for (noise, amount) in layers {
					total += noise.sample(x, y) * amount;
				}
				total.clamp(0.0, 1.0)
			},
		}
	}
}

pub struct SampleNoise {
	scale: f32,
	noise: OpenSimplexNoise,
}

impl SampleNoise {
	pub fn new(seed: u64, scale: f32) -> SampleNoise {
		SampleNoise {
			scale,
			// leo i felt like it.
			noise: OpenSimplexNoise::new(Some(unsafe { transmute(seed) })),
		}
	}

	pub fn sample(&self, x: f32, y: f32) -> f32 {
		((self
			.noise
			.eval_2d((x / self.scale) as f64, (y / self.scale) as f64) as f32)
			+ 1.0) / 2.0
	}
}
