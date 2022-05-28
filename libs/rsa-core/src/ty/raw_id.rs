#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub struct RawId(u32);

impl RawId {
	pub(crate) unsafe fn new(id: u32) -> RawId {
		RawId(id)
	}

	#[inline(always)]
	pub fn index(self) -> usize {
		self.0 as usize
	}
}
