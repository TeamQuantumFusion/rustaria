use std::fmt::Debug;
use mlua::UserData;
use serde::de::DeserializeOwned;

use crate::RawId;

pub trait Prototype: Clone + Send + Sync + 'static + UserData + DeserializeOwned + Debug {
    type Item;

    fn create(&self, id: RawId) -> Self::Item;
    fn name() -> &'static str;
}
