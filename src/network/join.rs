use rustaria_network::{EstablishingInstance, EstablishingStatus, Result};

pub struct PlayerJoinInstance {}

impl EstablishingInstance<PlayerJoinData> for PlayerJoinInstance {
	fn receive(&mut self, _data: &[u8]) -> Result<EstablishingStatus<PlayerJoinData>> {
		todo!()
	}
}

pub struct PlayerJoinData {}
