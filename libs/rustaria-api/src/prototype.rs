use std::{fmt::Debug, collections::HashSet};
use mlua::UserData;
use serde::de::DeserializeOwned;

use crate::{RawId, tag::Tag};

pub trait Prototype: Clone + Send + Sync + 'static + UserData + DeserializeOwned + Debug {
    type Item;

    fn create(&self, id: RawId) -> Self::Item;
    fn get_sprites(&self, sprites: &mut HashSet<Tag>) {}
    fn name() -> &'static str;
}
