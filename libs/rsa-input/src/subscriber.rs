pub enum Subscriber {
	Trigger(Box<dyn TriggerSubscriber>),
	Toggle(Box<dyn ToggleSubscriber>, bool),
	Hold(Box<dyn HoldSubscriber>),
}

pub trait TriggerSubscriber {
	fn pressed(&mut self);
	fn released(&mut self);
}

pub trait ToggleSubscriber {
	fn toggle(&mut self, state: bool);
}

pub trait HoldSubscriber {
	fn hold(&mut self, delta: f32);
}
