#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub struct RawId(pub u32);

impl RawId {
	#[inline(always)]
	pub fn index(self) -> usize {
		self.0 as usize
	}
}
