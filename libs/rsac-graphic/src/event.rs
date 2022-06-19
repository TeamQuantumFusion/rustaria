use std::time::Instant;
use glfw::{Action, Key, MouseButton, WindowEvent};
use rsa_core::ty::Direction;
use rsa_input::event::{Event, EventKind};
use rsa_input::event::keyboard::KeyboardEvent;
use rsa_input::event::mouse::{MouseEvent, ScrollEvent};

pub fn map_event(event: WindowEvent) -> Option<Event> {
	match event {
		WindowEvent::Pos(_, _) => {}
		WindowEvent::Size(_, _) => {}
		WindowEvent::Close => {}
		WindowEvent::Refresh => {}
		WindowEvent::Focus(_) => {}
		WindowEvent::Iconify(_) => {}
		WindowEvent::FramebufferSize(_, _) => {}
		WindowEvent::CursorPos(_, _) => {}
		WindowEvent::CursorEnter(_) => {}
		WindowEvent::Scroll(x, y) => {

			return Some(Event {
				kind: EventKind::Scroll(ScrollEvent {
					direction: if x > 0.0 {
						Direction::Right
					} else if x < 0.0 {
						Direction::Left
					} else if y > 0.0 {
						Direction::Up
					} else if y < 0.0 {
						Direction::Down
					} else {
						return None;
					},
				}),
				start: None,
				pressed: false,
			});
		}
		WindowEvent::MouseButton(button, action, modifiers) => {
			return Some(Event {
				kind: EventKind::Mouse(MouseEvent  {
					modifiers: map_modifiers(modifiers),
					button: map_button(button)
				}),
				start: Some(Instant::now()),
				pressed: map_action(action)?,
			});
		}
		WindowEvent::Key(key, _, action, modifiers) => {
			return Some(Event {
				kind: EventKind::Keyboard(KeyboardEvent  {
					modifiers: map_modifiers(modifiers),
					key: map_key(key)
				}),
				start: Some(Instant::now()),
				pressed: map_action(action)?,
			});
		}
		WindowEvent::Char(_) => {}
		WindowEvent::CharModifiers(_, _) => {}
		WindowEvent::FileDrop(_) => {}
		WindowEvent::Maximize(_) => {}
		WindowEvent::ContentScale(_, _) => {}
	}
	None
}

fn map_modifiers(action: glfw::Modifiers) -> rsa_input::event::modifier::Modifiers {
	rsa_input::event::modifier::Modifiers::from_bits_truncate(action.bits() as u8)
}

fn map_action(action: Action) -> Option<bool> {
	match action {
		Action::Release => Some(false),
		Action::Press => Some(true),
		Action::Repeat => None
	}
}

fn map_button(action: glfw::MouseButton) -> rsa_input::event::mouse::MouseButton {
	use  rsa_input::event::mouse::MouseButton as B;
	match action {
		MouseButton::Button1 => B::Left,
		MouseButton::Button2 => B::Middle,
		MouseButton::Button3 => B::Right,
		MouseButton::Button4 => B::Back,
		MouseButton::Button5 => B::Forward,
		MouseButton::Button6 => B::Other(6),
		MouseButton::Button7 => B::Other(7),
		MouseButton::Button8 => B::Other(8),
	}
}


fn map_key(action: glfw::Key) -> rsa_input::event::keyboard::Key {
	use rsa_input::event::keyboard::Key as K;
	match action {
		Key::Space => K::Char(' '),
		Key::Apostrophe => K::Char('\''),
		Key::Comma => K::Char(','),
		Key::Minus => K::Char('_'),
		Key::Period => K::Char('.'),
		Key::Slash => K::Char('/'),
		Key::Num0 => K::Char('0'),
		Key::Num1 => K::Char('1'),
		Key::Num2 => K::Char('2'),
		Key::Num3 => K::Char('3'),
		Key::Num4 => K::Char('4'),
		Key::Num5 => K::Char('5'),
		Key::Num6 => K::Char('6'),
		Key::Num7 => K::Char('7'),
		Key::Num8 => K::Char('8'),
		Key::Num9 => K::Char('9'),
		Key::Semicolon => K::Char(';'),
		Key::Equal => K::Char('='),
		Key::A => K::Char('a'),
		Key::B => K::Char('b'),
		Key::C => K::Char('c'),
		Key::D => K::Char('d'),
		Key::E => K::Char('e'),
		Key::F => K::Char('f'),
		Key::G => K::Char('g'),
		Key::H => K::Char('h'),
		Key::I => K::Char('i'),
		Key::J => K::Char('j'),
		Key::K => K::Char('k'),
		Key::L => K::Char('l'),
		Key::M => K::Char('m'),
		Key::N => K::Char('n'),
		Key::O => K::Char('o'),
		Key::P => K::Char('p'),
		Key::Q => K::Char('q'),
		Key::R => K::Char('r'),
		Key::S => K::Char('s'),
		Key::T => K::Char('t'),
		Key::U => K::Char('u'),
		Key::V => K::Char('v'),
		Key::W => K::Char('w'),
		Key::X => K::Char('x'),
		Key::Y => K::Char('y'),
		Key::Z => K::Char('z'),
		Key::LeftBracket => K::Char('{'),
		Key::Backslash => K::Char('\\'),
		Key::RightBracket => K::Char('}'),
		Key::GraveAccent => K::Char('`'),
		Key::Escape => K::Esc,
		Key::Enter => K::Enter,
		Key::Tab => K::Char('\t'),
		Key::Backspace => K::Backspace,
		Key::Insert => K::Insert,
		Key::Delete => K::Delete,
		Key::Right => K::Right,
		Key::Left => K::Left,
		Key::Down => K::Down,
		Key::Up => K::Up,
		Key::PageUp => K::PageUp,
		Key::PageDown => K::PageDown,
		Key::Home => K::Home,
		Key::End => K::End,
		Key::CapsLock => K::CapsLock,
		Key::ScrollLock => K::ScrollLock,
		Key::NumLock => K::NumLock,
		Key::PrintScreen => K::PrintScreen,
		Key::Pause => K::Pause,
		Key::F1 => K::Function(1),
		Key::F2 => K::Function(2),
		Key::F3 => K::Function(3),
		Key::F4 => K::Function(4),
		Key::F5 => K::Function(5),
		Key::F6 => K::Function(6),
		Key::F7 => K::Function(7),
		Key::F8 => K::Function(8),
		Key::F9 => K::Function(9),
		Key::F10 => K::Function(10),
		Key::F11 => K::Function(11),
		Key::F12 => K::Function(12),
		Key::F13 => K::Function(13),
		Key::F14 => K::Function(14),
		Key::F15 => K::Function(15),
		Key::F16 => K::Function(16),
		Key::F17 => K::Function(17),
		Key::F18 => K::Function(18),
		Key::F19 => K::Function(19),
		Key::F20 => K::Function(20),
		Key::F21 => K::Function(21),
		Key::F22 => K::Function(22),
		Key::F23 => K::Function(23),
		Key::F24 => K::Function(24),
		Key::F25 => K::Function(25),
		Key::Kp0 => K::KeyPad('0'),
		Key::Kp1 => K::KeyPad('1'),
		Key::Kp2 => K::KeyPad('2'),
		Key::Kp3 => K::KeyPad('3'),
		Key::Kp4 => K::KeyPad('4'),
		Key::Kp5 => K::KeyPad('5'),
		Key::Kp6 => K::KeyPad('6'),
		Key::Kp7 => K::KeyPad('7'),
		Key::Kp8 => K::KeyPad('8'),
		Key::Kp9 => K::KeyPad('9'),
		Key::KpDecimal => K::KeyPad('.'),
		Key::KpDivide => K::KeyPad('/'),
		Key::KpMultiply => K::KeyPad('*'),
		Key::KpSubtract => K::KeyPad('-'),
		Key::KpAdd => K::KeyPad('+'),
		Key::KpEnter => K::Enter,
		Key::KpEqual => K::KeyPad('='),
		Key::LeftShift => K::LeftShift,
		Key::LeftControl => K::LeftControl,
		Key::LeftAlt => K::LeftAlt,
		Key::LeftSuper => K::LeftSuper,
		Key::RightShift => K::RightShift,
		Key::RightControl => K::RightControl,
		Key::RightAlt => K::RightAlt,
		Key::RightSuper => K::RightSuper,
		Key::Menu => K::Menu,
		Key::Unknown => K::Unknown,
		Key::World1 => panic!("if you ever get this crash please contact me (!alpha) and tell me what key this is."),
		Key::World2 => panic!("if you ever get this crash please contact me (!alpha) and tell me what key this is."),
	}
}