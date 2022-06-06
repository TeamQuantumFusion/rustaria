pub enum Subscriber {
	Trigger {
		press: Box<dyn FnMut()>,
		release: Box<dyn FnMut()>,
	},
	Toggle {
		value: bool,
		toggle: Box<dyn FnMut(bool)>
	},
	Hold {
		hold: Box<dyn FnMut(f32)>
	},
}

impl Subscriber {
	pub fn press(press: impl FnMut() + 'static) -> Subscriber {
		Subscriber::Trigger {
			press: Box::new(press),
			release: Box::new(|| {}),
		}
	}

	pub fn release(release: impl FnMut() + 'static) -> Subscriber {
		Subscriber::Trigger {
			release: Box::new(release),
			press: Box::new(|| {}),
		}
	}

	pub fn trigger(release: impl FnMut() + 'static, press: impl FnMut() + 'static) -> Subscriber {
		Subscriber::Trigger {
			release: Box::new(release),
			press: Box::new(press),
		}
	}

	pub fn toggle(default: bool, toggle: impl FnMut(bool) + 'static) -> Subscriber {
		Subscriber::Toggle {
			value: default,
			toggle: Box::new(toggle)
		}
	}

	pub fn hold(hold: impl FnMut(f32) + 'static) -> Subscriber {
		Subscriber::Hold {
			hold: Box::new(hold)
		}
	}
}