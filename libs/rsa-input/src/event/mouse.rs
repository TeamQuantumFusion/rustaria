use rsa_core::ty::Direction;
use crate::event::modifier::Modifiers;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MouseEvent {
	pub modifiers: Modifiers,
	pub button: MouseButton,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MouseButton {
	Left,
	Right,
	Middle,
	Back,
	Forward,
	Other(u8)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ScrollEvent {
	pub direction: Direction
}