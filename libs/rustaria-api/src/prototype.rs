use crate::RawId;
use mlua::UserData;
use serde::de::DeserializeOwned;

pub trait Prototype: Clone + Send + Sync + 'static + UserData + DeserializeOwned {
    type Item;

    fn create(&self, id: RawId) -> Self::Item;
    fn name() -> &'static str;
}
