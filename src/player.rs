#![allow(unused)] // alpha, remove this when you're done - leocth

pub struct Player {
    x: f32,
    y: f32,
    name: String
}

impl Player {
    pub fn new(x: f32, y: f32, name: String) -> Self {
        Self {
            x,
            y,
            name
        }
    }
}