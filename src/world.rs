#![allow(unused)] // alpha, remove this when you're done - leocth

use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use eyre::ContextCompat;
use crate::api::{Prototype, Rustaria};

use crate::chunk::Chunk;
use crate::chunk::tile::Tile;
use crate::player::Player;
use crate::registry::Id;
use crate::types::{ChunkPos, TilePos};

pub struct World {
    // size is chunks x chunks
    chunk_size: (u32, u32),
    // 4x4 chunk grid look like this in the vec
    // y[x,x,x,x], y[x,x,x,x], y[x,x,x,x], y[x,x,x,x]
    chunks: Vec<Chunk>,
    players: HashMap<PlayerId, Player>,
    event_queue: VecDeque<Command>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(u16);

impl World {
    pub fn new(chunk_size: (u32, u32), chunks: Vec<Chunk>) -> Result<World, WorldCreationError> {
        if chunk_size.0 as usize * chunk_size.1 as usize != chunks.len() {
            return Err(WorldCreationError::InvalidWorldSize);
        }

        Ok(Self {
            chunk_size,
            chunks,
            players: HashMap::new(),
            event_queue: VecDeque::new()
        })
    }

    pub fn player_join(&mut self, player: Player) -> PlayerId {
        let id = PlayerId(self.players.len() as u16);
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
        let (world_w, world_h) = self.chunk_size;
        let internal_x = pos.x.checked_add(world_w as i32 / 2)? as u32;
        let internal_y = pos.y;

        if internal_x > world_w || internal_y > world_h {
            return None;
        }

        self.chunks
            .get((internal_y as usize * world_h as usize) + internal_x as usize)
    }

    fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        let (world_w, world_h) = self.chunk_size;
        let internal_x = pos.x.checked_add(world_w as i32 / 2)? as u32;
        let internal_y = pos.y;

        if internal_x > world_w || internal_y > world_h {
            return None;
        }

        self.chunks.get_mut((internal_y as usize * world_h as usize) + internal_x as usize)
    }


    pub fn tick(&mut self, rustaria: &Rustaria) {
        while let Some(event) = self.event_queue.pop_back() {
            event.execute(&mut self, rustaria);
        }
    }

    pub fn send_event(&mut self, command: Command) {
        self.event_queue.push_front(command);
    }
}

#[derive(Copy, Clone)]
pub enum Command {
    SetTile(Id, TilePos),
}

impl Command {
    pub fn execute(&self, world: &mut World, rustaria: &Rustaria) -> eyre::Result<()>{
        match self {
            Command::SetTile(id, pos) => {
                if let Some(chunk) = world.get_chunk_mut(pos.chunk_pos()) {
                    let prototype = rustaria.tiles.get_entry(id).wrap_err("Could not find id.");
                    chunk.tiles.set(pos.sub_pos(), prototype?.create(*id));
                }
            }
        }
        Ok(())
    }
}

// NOTE(leocth): `thiserror` might be appropriate here
#[derive(Debug)]
pub enum WorldCreationError {
    InvalidWorldSize,
}

impl Display for WorldCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldCreationError::InvalidWorldSize => write!(
                f,
                "InvalidWorldSize: Chunk size does not match Chunk Storage."
            ),
        }
    }
}

impl std::error::Error for WorldCreationError {}
