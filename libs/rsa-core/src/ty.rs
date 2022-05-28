//! A collection of types used in Rustaria.

mod chunk_pos;
mod chunk_sub_pos;
mod direction;
mod plugin_id;
mod pos;
mod raw_id;
mod rectangle;
mod tag;
mod tile_pos;

use std::collections::HashSet;
use std::fmt::Debug;
use crate::api::lua::FromLua;
// Reexport
pub use {
	chunk_pos::ChunkPos, chunk_sub_pos::ChunkSubPos, direction::Direction, plugin_id::PluginId,
	pos::Pos, raw_id::RawId, rectangle::Rectangle, tag::Tag, tile_pos::TilePos, uuid::Uuid,
};

pub enum Error {
	OutOfBounds,
}

use crate::settings::CHUNK_SIZE;
pub const CHUNK_SIZE_U8: u8 = CHUNK_SIZE as u8;
pub const CHUNK_SIZE_MASK: u8 = (CHUNK_SIZE - 1) as u8;
pub const CHUNK_SIZE_F: f32 = CHUNK_SIZE as f32;

// lets later implement corner directions.
pub trait Offset<D>: Sized {
	fn wrapping_offset(self, displacement: D) -> Self;
	fn checked_offset(self, displacement: D) -> Option<Self>;
}

pub trait Prototype: Send + Clone + Sync + FromLua + 'static + Debug {
	type Item;

	fn create(&self, id: RawId) -> Self::Item;
	fn get_sprites(&self, _sprites: &mut HashSet<Tag>) {}
	fn lua_registry_name() -> &'static str;
}

#[inline]
fn checked_add_signed_u32(a: u32, b: i32) -> Option<u32> {
	// XXX(leocth):
	// replace with std's `checked_add_signed` when `mixed_integer_ops` reaches stable.
	// see https://github.com/rust-lang/rust/issues/87840
	let (res, overflowed) = a.overflowing_add(b as u32);
	if overflowed ^ (b < 0) {
		None
	} else {
		Some(res)
	}
}

#[inline]
fn checked_add_signed_u8(a: u8, b: i8) -> Option<u8> {
	// XXX(leocth):
	// replace with std's `checked_add_signed` when `mixed_integer_ops` reaches stable.
	// see https://github.com/rust-lang/rust/issues/87840
	let (res, overflowed) = a.overflowing_add(b as u8);
	if overflowed ^ (b < 0) {
		None
	} else {
		Some(res)
	}
}
