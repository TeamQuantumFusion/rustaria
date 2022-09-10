use std::fmt::{Display, Formatter};

pub use self::direction::{DirMap, Direction};

mod direction;

/// World Space
pub struct WS;

/// Screen Space
pub struct SS;

#[derive(Debug)]
pub enum Error {
	OutOfBounds,
}

impl std::error::Error for Error {}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("Coordinates are out of bounds")
	}
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
pub fn checked_add_signed_u32(a: u32, b: i32) -> Option<u32> {
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
pub fn checked_add_signed_u8(a: u8, b: i8) -> Option<u8> {
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
