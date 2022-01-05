use crate::chunk::Chunk;
use crate::player::Player;

pub struct World {
    // size is chunks x chunks
    size: (u32, u32),
    // 4x4 chunk grid look like this in the vec
    // y[x,x,x,x], y[x,x,x,x], y[x,x,x,x], y[x,x,x,x]
    chunks: Vec<Chunk>,
    // Every player has an id that they get when they join which is the index in the vec.
    // When a player leaves the index of the player will become None
    // Which preserves order and allows persistent Player identification.
    players: Vec<Option<Player>>,
}

pub struct ChunkPos {
    x: i32,
    y: u32,
}

pub struct PlayerId {
    index: usize,
}

impl World {
    pub fn new(size: (u32, u32), chunks: Vec<Chunk>) -> Result<World, WorldCreationError> {
        if size.0 as usize * size.1 as usize != chunks.len() {
            return Err(WorldCreationError::InvalidWorldSize);
        }

        Ok(Self {
            size,
            chunks,
            players: vec![],
        })
    }

    pub fn player_join(&mut self, player: Player) -> PlayerId {
        let index = self.players.len();
        self.players.push(Some(player));
        PlayerId { index }
    }

    pub fn player_leave(&mut self, player_id: PlayerId) {
        self.players.insert(player_id.index, None);
    }

    pub fn get_player(&self, player_id: PlayerId) -> Option<&Player> {
        if let Some(Some(player)) = self.players.get(player_id.index) {
            Some(player)
        } else {
            None
        }
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        let world_w = self.size.0;
        let world_h = self.size.1;
        let internal_x = pos.x.checked_add(world_w as i32 / 2)? as u32;
        let internal_y = pos.y;

        if internal_x > world_w || internal_y > world_h {
            return None;
        }

        self.chunks.get((internal_y as usize * world_h as usize) + internal_x as usize)
    }
}

pub enum WorldCreationError {
    InvalidWorldSize
}