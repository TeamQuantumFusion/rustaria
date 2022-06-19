pub mod block_layer_pos;
pub mod block_pos;
pub mod chunk_pos;
pub mod direction;
pub mod id;
pub mod identifier;

/// World Space
pub struct WS;
/// Screen Space
pub struct SS;

pub enum Error {
	OutOfBounds,
}

pub trait MultiDeref<T> {
	fn get_child(&self) -> &T;
	fn get_mut_child(&mut self) -> &mut T;
}

#[macro_export]
macro_rules! multi_deref_fields {
    ($FOR:ty {$($IDENT:ident:$TY:ty),*}) => {
	    $(
	    impl MultiDeref<$TY> for $FOR {
			fn get_child(&self) -> &$TY {
				&self.$IDENT
			}
            fn get_mut_child(&mut self) -> &mut $TY {
				&mut self.$IDENT
			}
		}
	    )*
    };
}

pub trait Offset<D>: Sized {
	fn wrapping_offset(self, displacement: D) -> Self;
	fn checked_offset(self, displacement: D) -> Option<Self>;
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
