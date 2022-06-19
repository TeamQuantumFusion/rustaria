pub enum Subscriber {
	Trigger {
		press: Box<dyn FnMut(f64)>,
		release: Box<dyn FnMut(f64)>,
	},
	Toggle {
		value: bool,
		toggle: Box<dyn FnMut(f64, bool)>
	},
	Hold {
		hold: Box<dyn FnMut(f64, f32)>
	},
}

impl Subscriber {
	pub fn press(press: impl FnMut(f64) + 'static) -> Subscriber {
		Subscriber::Trigger {
			press: Box::new(press),
			release: Box::new(|_| {}),
		}
	}

	pub fn release(release: impl FnMut(f64) + 'static) -> Subscriber {
		Subscriber::Trigger {
			release: Box::new(release),
			press: Box::new(|_| {}),
		}
	}

	pub fn trigger(release: impl FnMut(f64) + 'static, press: impl FnMut(f64) + 'static) -> Subscriber {
		Subscriber::Trigger {
			release: Box::new(release),
			press: Box::new(press),
		}
	}

	pub fn toggle(default: bool, toggle: impl FnMut(f64, bool) + 'static) -> Subscriber {
		Subscriber::Toggle {
			value: default,
			toggle: Box::new(toggle)
		}
	}

	pub fn hold(hold: impl FnMut(f64, f32) + 'static) -> Subscriber {
		Subscriber::Hold {
			hold: Box::new(hold)
		}
	}
}