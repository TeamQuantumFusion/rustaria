use rustaria_api::ty::RawId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Tile {
	pub id: RawId,
	pub collision: bool,
	pub opaque: bool,
}
