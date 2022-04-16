use std::collections::HashMap;

use mlua::UserData;
use serde::{Deserialize, Serialize};

use rustaria_api::ty::Tag;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RenderingSystem {
    Static(Pane),
    State(HashMap<String, Pane>),
    // More implementations for dynamic lua rendering.
    // Advanced(stuff)
}

impl UserData for RenderingSystem {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pane {
    pub x_offset: f32,
    pub y_offset: f32,
    pub width: f32,
    pub height: f32,
    pub sprite: Tag,
}
