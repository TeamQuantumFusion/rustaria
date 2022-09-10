use apollo::Function;
use apollo::FromLua;

#[derive(FromLua, Debug, Clone)]
pub struct BlockStates {
	pub states: Vec<BlockState>,
	pub resolution: Function,
	pub default_state: u8,
}

#[derive(FromLua, Debug, Clone)]
pub struct BlockState {
	pub name: String
}