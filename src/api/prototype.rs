use std::fmt::Debug;
use apollo::{FromLua};

pub trait Prototype
where
	Self: Sized + 'static + Send + FromLua + Debug,
{
	type Output;
	fn get_name() -> &'static str;
}
