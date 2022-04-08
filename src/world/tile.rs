use rustaria_api::RawId;

pub struct Tile {
	pub id: RawId,
	pub collision: bool,
	pub opaque: bool,
}