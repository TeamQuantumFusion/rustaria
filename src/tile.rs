use serde::{Deserialize, Serialize};
use rsa_core::ty::RawId;


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Tile {
	pub id: RawId,
	pub collision: bool,
	pub opaque: bool,
}
