use rsa_core::ty::Uuid;

#[derive(Clone)]
pub struct Player {
	pub name: String,
	pub entity: Option<Uuid>,

	// Jump
	pub jump_frames: u32,
	pub jump_speed: f32,

	// Run
	pub run_acceleration: f32,
	pub run_slowdown: f32,
	pub run_max_speed: f32,
}
impl Player {
	pub fn new(name: String) -> Player {
		Player {
			name,
			entity: None,
			jump_frames: 15,
			jump_speed: 20.0,
			run_acceleration: 4.8,
			run_slowdown: 1.2,
			run_max_speed: 12.0,
		}
	}
}
