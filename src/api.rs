use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard};

use rustaria_api::ApiHandler;
use rustaria_api::lua_runtime::Lua;
use rustaria_api::tag::Tag;
use rustaria_util::Result;

#[macro_use]
pub mod prototype;
pub mod ty;

#[derive(Clone)]
pub struct Api {
    instance: Arc<RwLock<ApiHandler>>,
}

impl Api {
    pub fn new(lua: &Lua) -> Api {
        Api {
            instance: Arc::new(RwLock::new(ApiHandler::new(lua).unwrap())),
        }
    }

    pub fn reload(&mut self, lua: &Lua) -> Result<()> {
        let mut write = self.instance.write().unwrap();
        let mut reload = write.reload();
        prototypes!({ reload.register_builder::<P>(lua)? });
        reload.reload(lua)?;
        prototypes!({ reload.compile_builder::<P>(lua)? });
        reload.apply();
        Ok(())
    }

    pub fn instance(&self) -> RwLockReadGuard<'_, ApiHandler> {
        self.instance.read().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name;

    #[test]
    fn test() {
        prototypes!({ println!("{}", type_name::<P>()) });
    }
}
