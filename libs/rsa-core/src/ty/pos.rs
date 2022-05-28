use euclid::{vec2, Vector2D};

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
#[use_default]
pub struct Pos {
	pub x: f32,
	pub y: f32,
}

#[allow(clippy::from_over_into)]
impl<S> Into<Vector2D<f32, S>> for Pos {
	fn into(self) -> Vector2D<f32, S> {
		vec2(self.x, self.y)
	}
}
