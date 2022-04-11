use std::collections::HashMap;
use rustaria_api::tag::Tag;
use serde::Deserialize;
use rustaria_api::lua_runtime::UserData;

#[derive(Clone, Debug, Deserialize)]
pub enum RenderingSystem {
	Static(Pane),
	State(HashMap<String, Pane>),
	// More implementations for dynamic lua rendering.
	// Advanced(stuff)
}

impl UserData for RenderingSystem {}

#[derive(Clone, Debug, Deserialize)]
pub struct Pane {
	pub x_offset: f32,
	pub y_offset: f32,
	pub width: f32,
	pub height: f32,
	pub sprite: Tag
}

impl UserData for Pane {}
