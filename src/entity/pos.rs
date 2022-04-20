use rustaria_util::ty::pos::Pos;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PositionComp {
	pub position: Pos,
}
