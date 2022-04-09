use crate::RawId;
use mlua::UserData;
use serde::de::DeserializeOwned;

pub trait Prototype: UserData + Clone + DeserializeOwned + 'static {
    type Item;

    fn create(&self, id: RawId) -> Self::Item;
    fn name() -> &'static str;
}
