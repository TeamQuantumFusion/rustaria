#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ControllerButtonEvent {
	button: ControllerButton
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]

pub enum ControllerButton {
	// Controller triggers
	TriggerLeftTop,
	TriggerRightTop,
	TriggerLeftBottom,
	TriggerRightBottom,
	// Controller front buttons
	// Dial
	DialUp,
	DialLeft,
	DialDown,
	DialRight,
	// Joystick
	JoystickLeft,
	JoystickRight,
	// Dpad
	DPadUp,
	DPadLeft,
	DPadDown,
	DPadRight,
}