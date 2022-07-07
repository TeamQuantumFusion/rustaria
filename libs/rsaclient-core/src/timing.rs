use std::time::{Duration, Instant};
use rsa_core::TPS;

pub const TICK_DURATION: Duration = Duration::from_nanos((1000000000 / TPS) as u64);

pub struct Timing {
	last_tick: Instant,
	tick_delta: f32,
	old_delta: f32,

	// cache
	ticked: bool,
	step: f32,
}

impl Timing {
	pub fn new() -> Timing {
		Timing {
			last_tick: Instant::now(),
			tick_delta: 0.0,
			old_delta: 0.0,
			ticked: false,
			step: 0.0,
		}
	}

	pub fn next_tick(&mut self) -> bool {
		if let Some(value) = Instant::now().checked_duration_since(self.last_tick) {
			if value > TICK_DURATION {
				self.last_tick += TICK_DURATION;
				return true;
			} else {
				// we are about to draw a frame
				self.next((value.as_secs_f64() / TICK_DURATION.as_secs_f64()) as f32);
			}
		}

		false
	}

	// used for testing too
	fn next(&mut self, tick_delta: f32) {
		self.old_delta = self.tick_delta;
		self.tick_delta = tick_delta;

		self.ticked = self.tick_delta < self.old_delta;
		self.step = if self.ticked {
			(1.0 - self.old_delta) + self.tick_delta
		} else {
			self.tick_delta - self.old_delta
		};
	}

	#[inline(always)]
	pub fn ticked(&self) -> bool { self.ticked }

	#[inline(always)]
	pub fn delta(&self) -> f32 { self.tick_delta }

	#[inline(always)]
	pub fn step(&self) -> f32 { self.step }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut timing = Timing::new();
		timing.next(0.7);
		timing.next(0.1);
		assert!(timing.ticked());
		assert_eq!(timing.delta(), 0.1);
		assert_eq!(timing.step(), 0.4);
		timing.next(0.4);
		assert_eq!(timing.delta(), 0.4);
		assert_eq!(timing.step(), 0.3);
	}
}
