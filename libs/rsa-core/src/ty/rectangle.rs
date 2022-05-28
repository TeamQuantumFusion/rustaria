use euclid::{rect, Rect};

#[derive(
	Copy,
	Clone,
	PartialOrd,
	PartialEq,
	Debug,
	Default,
	serde::Serialize,
	serde::Deserialize,
	frogelua::FromLua,
)]
pub struct Rectangle {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

#[allow(clippy::from_over_into)]
impl<S> Into<Rect<f32, S>> for Rectangle {
	fn into(self) -> Rect<f32, S> {
		rect(self.x, self.y, self.width, self.height)
	}
}
