use crate::sweep::sampler::Sampler;
use crate::util::pass::WorldPass;
use opensimplex_noise_rs::OpenSimplexNoise;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro128StarStar;
use rustaria_common::ty::Direction;
use std::cell::UnsafeCell;
use std::ops::Range;

pub struct NoiseGenerator {
	pub simplex: OpenSimplexNoise,
	pub rng: UnsafeCell<Xoshiro128StarStar>,
}

impl NoiseGenerator {
	pub fn new(seed: u32) -> NoiseGenerator {
		NoiseGenerator {
			// leo i felt like it.
			simplex: OpenSimplexNoise::new(Some(seed)),
			rng: UnsafeCell::new(Xoshiro128StarStar::seed_from_u64(seed as u64)),
		}
	}

	pub fn rng(&self) -> &mut Xoshiro128StarStar {
		unsafe { &mut *self.rng.get() }
	}
}

#[derive(Clone)]
pub enum Noise {
	/// Returns {0} no matter where on the world the point is sampled.
	Const(f32),
	/// Clamped extends the inner samplers range.
	/// - anything below `start` will be 0,
	/// - anything above `end` will be 1,
	/// - anything between `start`..`end` will be scaled to those values. so a inner of 0.5 will give the value that is between `start` and `end`
	Clamped(Range<f32>, Box<Noise>),

	Gradient(f32, f32, f32),
	// only really works 0..1 x and y
	// 0 is fully noise while 1 is fully 1.
	FadeTo(Direction, Range<f32>, f32, Box<Noise>),
	Y,
	X,
	Random,
}

impl Noise {
	pub fn constant(value: f32) -> Noise {
		Noise::Const(value)
	}

	pub fn simplex(scale_x: f32, scale_y: f32) -> Noise {
		Noise::Gradient(scale_x, scale_y, 0.0)
	}

	pub fn simplex_offset(scale: f32, offset: f32) -> Noise {
		Noise::Gradient(scale, scale, offset)
	}

	pub fn clamp(range: Range<f32>, noise: Noise) -> Noise {
		Noise::Clamped(range, Box::new(noise))
	}

	pub fn fade(dir: Direction, range: Range<f32>, to: f32, from: Noise) -> Noise {
		Noise::FadeTo(dir, range, to, Box::new(from))
	}
}

#[macro_export]
macro_rules! noise {
    (literal $VALUE:literal) => {
		Noise::Static($VALUE)
    };
	(random) => {
		Noise::Random
    };
	(gradient $SCALE:literal$(@$OFFSET:literal)?) => {
		Noise::Gradient($SCALE, $SCALE, 0.0 $(+ $OFFSET)?)
    };
	(clamp $CLAMP:expr => $NOISE:expr) => {
		Noise::Clamped($CLAMP, Box::new($NOISE))
	};
	(fade [$DIR:expr => $TO:literal] => $NOISE:expr) => {
		Noise::FadeTo($DIR, 0.0..1.0, $TO, Box::new($NOISE))
	};
	(fade [$RANGE:expr, $DIR:expr => $TO:literal] => $NOISE:expr) => {
		Noise::FadeTo($DIR, $RANGE, $TO, Box::new($NOISE))
	};
}

#[cfg(test)]
mod fuck_off {

	#[test]
	pub fn test() {}
}
