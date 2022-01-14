#![allow(unused)] // alpha, remove this when you're done - leocth

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::chunk::Chunk;
use crate::player::Player;

pub struct World {
    // size is chunks x chunks
    chunk_size: (u32, u32),
    // 4x4 chunk grid look like this in the vec
    // y[x,x,x,x], y[x,x,x,x], y[x,x,x,x], y[x,x,x,x]
    chunks: Vec<Chunk>,
    players: HashMap<PlayerId, Player>,
}

pub struct ChunkPos {
    x: i32,
    y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(usize);

impl World {
    pub fn new(chunk_size: (u32, u32), chunks: Vec<Chunk>) -> Result<World, WorldCreationError> {
        if chunk_size.0 as usize * chunk_size.1 as usize != chunks.len() {
            return Err(WorldCreationError::InvalidWorldSize);
        }

        Ok(Self {
            chunk_size,
            chunks,
            players: HashMap::new(),
        })
    }

    pub fn player_join(&mut self, player: Player) -> PlayerId {
        let id = PlayerId(self.players.len());
        self.players.insert(id, player);
        id
    }

    pub fn player_leave(&mut self, player_id: PlayerId) {
        self.players.remove(&player_id);
    }

    pub fn get_player(&self, player_id: PlayerId) -> Option<&Player> {
        self.players.get(&player_id)
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        let world_w = self.chunk_size.0;
        let world_h = self.chunk_size.1;
        let internal_x = pos.x.checked_add(world_w as i32 / 2)? as u32;
        let internal_y = pos.y;

        if internal_x > world_w || internal_y > world_h {
            return None;
        }

        self.chunks
            .get((internal_y as usize * world_h as usize) + internal_x as usize)
    }
}

#[derive(Debug)]
pub enum WorldCreationError {
    InvalidWorldSize,
}

impl Display for WorldCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldCreationError::InvalidWorldSize =>  write!(f, "InvalidWorldSize: Chunk size does not match Chunk Storage."),
        }
    }
}