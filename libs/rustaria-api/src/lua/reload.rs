use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

use mlua::Lua;
use rustaria_util::{blake3::Hasher, trace};

use crate::{
	registry::Registry,
	ty::{Prototype, RawId, Tag},
};

#[derive(Clone)]
pub struct RegistryBuilder<P: Prototype> {
	entries: Arc<Mutex<HashMap<Tag, P>>>,
}

impl<P: Prototype> RegistryBuilder<P> {
	pub fn register(&mut self, lua: &Lua) -> mlua::Result<()> {
		lua.globals().set(P::lua_registry_name(), self.clone())
	}

	pub fn finish(self, hasher: &mut Hasher) -> mlua::Result<Registry<P>> {
		let mut entries = self.entries.lock();
		let data = std::mem::take(&mut *entries);
		let mut data: Vec<_> = data.into_iter().collect();

		data.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

		for (id, (tag, _)) in data.iter().enumerate() {
			hasher.update(&id.to_be_bytes());
			hasher.update(tag.as_bytes());
		}

		let mut tag_to_id = HashMap::new();
		let mut id_to_tag = Vec::new();
		let mut entries = Vec::new();

		for (id, (tag, prototype)) in data.into_iter().enumerate() {
			tag_to_id.insert(tag.clone(), RawId(id as u32));
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
impl<P: Prototype> Default for RegistryBuilder<P> {
	fn default() -> Self {
		Self {
			entries: Default::default(),
		}
	}
}

impl<P: Prototype> mlua::UserData for RegistryBuilder<P> {
	fn add_methods<M: mlua::UserDataMethods<Self>>(m: &mut M) {
		m.add_method_mut("register", |_, this, prototypes: HashMap<Tag, P>| {
			trace!(
				target: P::lua_registry_name(),
				"Registered entries to registry"
			);

			let mut entries = this.entries.lock();
			for (tag, prototype) in prototypes {
				trace!(
					target: P::lua_registry_name(),
					"Registered: {tag} {prototype:?}"
				);
				entries.insert(tag, prototype);
			}
			Ok(())
		});
	}
}
