use rustaria_util::Uuid;

pub struct Player {
	pub name: String,
	pub entity: Option<Uuid>,
	// inventory whatever
}
impl Player {
	pub fn new(name: String) -> Player {
		Player { name, entity: None }
	}
}
