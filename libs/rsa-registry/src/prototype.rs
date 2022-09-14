use std::fmt::Debug;
use apollo::FromLua;
use crate::IdValue;

pub trait Prototype
	where
		Self: Sized + 'static + Send + FromLua + Debug,
{
	type Output: IdValue;
	fn get_name() -> &'static str;
}

impl<O: IdValue, P: Prototype<Output = O>> IdValue for P {
	type Idx = O::Idx;
}