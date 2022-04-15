use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use mlua::{Lua, LuaSerdeExt, Table, UserData, Value};
use rustaria_util::{blake3::Hasher, info, trace};

use crate::{
    registry::Registry,
    ty::{LuaConvertableCar, Prototype, RawId, Tag, LuaCar},
};

#[derive(Clone)]
pub struct RegistryBuilder<P: Prototype + LuaConvertableCar> {
    entries: Arc<Mutex<Option<HashMap<Tag, P>>>>,
}

impl<P: Prototype + LuaConvertableCar> RegistryBuilder<P> {
    pub fn new() -> RegistryBuilder<P> {
        RegistryBuilder {
            entries: Arc::new(Mutex::new(Some(HashMap::new()))),
        }
    }

    pub fn register(&mut self, lua: &Lua) -> mlua::Result<()> {
        lua.globals().set(P::lua_registry_name(), self.clone())
    }

    pub fn finish(self, hasher: &mut Hasher) -> mlua::Result<Registry<P>> {
        let mut data: Vec<(Tag, P)> = self
            .entries
            .lock()
            .unwrap()
            .take()
            .unwrap()
            .into_iter()
            .collect();

        data.sort_by(|(i1, _), (i2, _)| i1.to_string().cmp(&i2.to_string()));

        for (id, (tag, _)) in data.iter().enumerate() {
            hasher.update(&id.to_be_bytes());
            hasher.update(tag.to_string().as_bytes());
        }

        let mut tag_to_id = HashMap::new();
        let mut id_to_tag = Vec::new();
        let mut entries = Vec::new();

        for (id, (tag, prototype)) in data.into_iter().enumerate() {
            tag_to_id.insert(tag.clone(), RawId::new(id as u32));
            id_to_tag.push(tag);
            entries.push(prototype);
        }

        Ok(Registry {
            tag_to_id,
            id_to_tag,
            entries,
        })
    }
}

impl<P: Prototype + LuaConvertableCar> mlua::UserData for RegistryBuilder<P> {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(m: &mut M) {
        m.add_method_mut("register", |lua, this, t: Value| {
            trace!(
                target: P::lua_registry_name(),
                "Registered entries to registry"
            );

            let new_entries: HashMap<Tag, Value> = lua.unpack(t)?;
            let mut lock = this.entries.lock().unwrap();
            let entries = lock.as_mut().unwrap();
            for (tag, table) in new_entries {
                trace!(target: P::lua_registry_name(), "Registering: {tag} {table:?}");
                let prototype: P = P::from_luaagh(table, lua)?;
                trace!(target: P::lua_registry_name(), "Registered: {tag} {prototype:?}");
                entries.insert(tag, prototype);
            }
            Ok(())
        });
    }
}
