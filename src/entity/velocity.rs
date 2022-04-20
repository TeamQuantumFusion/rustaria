use crate::entity::pos::PositionComp;
use rustaria_util::ty::pos::Pos;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct VelocityComp {
	pub velocity: Pos,
}

impl VelocityComp {
	pub fn tick(&mut self, pos: &mut PositionComp) {
		pos.position += self.velocity;
	}
}

impl Default for VelocityComp {
	fn default() -> Self {
		VelocityComp {
			velocity: Pos { x: 0.0, y: 0.0 },
		}
	}
}
