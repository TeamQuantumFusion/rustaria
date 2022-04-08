use rustaria_network::{EstablishingInstance, EstablishingStatus};
use rustaria_util::Result;

pub struct PlayerJoinInstance {}

impl EstablishingInstance<PlayerJoinData> for PlayerJoinInstance {
	fn receive(&mut self, data: &[u8]) -> Result<EstablishingStatus<PlayerJoinData>> {
		todo!()
	}
}

pub struct PlayerJoinData {}
