pub mod controller_button;
pub mod keyboard;
pub mod modifier;
pub mod mouse;

#[cfg(test)]
use mock_instant::Instant;
use rsa_core::ty::Direction;
use std::time::Duration;

#[cfg(not(test))]
use std::time::Instant;

use crate::event::controller_button::ControllerButtonEvent;
use crate::event::keyboard::KeyboardEvent;
use crate::event::mouse::{MouseEvent, ScrollEvent};
use rsa_core::settings::UPS;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Event {
	pub kind: EventKind,
	pub start: Option<Instant>,
	pub pressed: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EventRecord {
	pub kind: EventKind,
	pub duration: Option<Duration>,
	pub state: EventState,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EventKind {
	Keyboard(KeyboardEvent),
	Mouse(MouseEvent),
	ControllerButton(ControllerButtonEvent),
	Scroll(ScrollEvent),
}

impl EventKind {
	pub fn sustained(&self) -> bool {
		match self {
			EventKind::Keyboard(_) | EventKind::Mouse(_) | EventKind::ControllerButton(_) => true,
			EventKind::Scroll(_) => false,
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Default)]
pub struct EventState {
	pub fire_press: bool,
	pub fire_release: bool,
}
impl EventState {
	pub fn full() -> EventState {
		EventState {
			fire_press: true,
			fire_release: true,
		}
	}
}

impl EventRecord {
	pub fn new_dual(start_action: Event, end_action: Event, state: EventState) -> EventRecord {
		EventRecord {
			kind: start_action.kind,
			duration: match (start_action.start, end_action.start) {
				(Some(start), Some(end)) => Some(end.saturating_duration_since(start)),
				_ => None,
			},
			state,
		}
	}

	pub fn new(action: Event, state: EventState) -> EventRecord {
		EventRecord {
			kind: action.kind,
			duration: None,
			state,
		}
	}

	pub fn get_delta(&self) -> f32 {
		match self.duration {
			Some(duration) => ((duration.as_secs_f32() * UPS as f32) % 1.0).clamp(0.0, 1.0),
			None => 1.0,
		}
	}
}
