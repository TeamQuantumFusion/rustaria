use crate::event::modifier::Modifiers;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyboardEvent {
	pub modifiers: Modifiers,
	pub key: Key,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Key {
	// Keys
	Char(char),
	// F1-12-24-whatever
	Function(u8),
	KeyPad(char),
	//
	CapsLock,
	ScrollLock,
	NumLock,
	LeftShift,
	LeftControl,
	LeftAlt,
	LeftSuper,
	RightShift,
	RightControl,
	RightAlt,
	RightSuper,
	Menu,
	//
	Backspace,
	Enter,
	Esc,
	PrintScreen,
	Pause,
	//
	Insert,
	Home,
	PageUp,
	Delete,
	End,
	PageDown,
	//
	Up,
	Down,
	Left,
	Right,
	//
	Unknown,
}
