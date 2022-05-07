use crate::Noise;

// Selects a value. (u16)
pub struct Selector {
	pub(crate) noise: Noise,
	pub(crate) values: Vec<(f32, u16)>
}