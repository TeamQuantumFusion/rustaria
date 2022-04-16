use std::collections::HashSet;
use mlua::UserData;
use serde::{Deserialize, Serialize};

use rustaria_api::ty::{LuaCar, LuaConvertableCar, Tag};
use rustaria_api::ty::{Prototype, RawId};
use rustaria_util::ty::pos::Pos;

use crate::api::rendering::RenderingSystem;
use crate::entity::VelocityComp;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityPrototype {
    pub velocity: Option<VelocityCompPrototype>,
    #[cfg(feature = "client")]
    pub rendering: Option<RenderingSystem>,
}

impl LuaConvertableCar for EntityPrototype {
    fn from_luaagh(table: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::Table(table) = table {
            Ok(EntityPrototype {
                velocity: table.get::<_, LuaCar<_>>("velocity")?.0,
                rendering: table.get::<_, LuaCar<_>>("rendering")?.0,
                // collision: table.get("collision")?,
                // opaque: table.get("opaque")?,
                // blast_resistance: table.get("blast_resistance")?,
                // break_resistance: table.get("break_resistance")?,
            })
        } else {
            Err(mlua::Error::UserDataTypeMismatch)
        }
    }

    fn into_luaagh(self, _: &mlua::Lua) -> mlua::Result<mlua::Value> {
        todo!()
    }
}

impl Prototype for EntityPrototype {
    type Item = ();

    fn create(&self, _: RawId) -> Self::Item {}

    fn get_sprites(&self,sprites: &mut HashSet<Tag>) {
        if let Some(system) = &self.rendering {
            match system {
                RenderingSystem::Static(pane) => {
                    sprites.insert(pane.sprite.clone());
                }
                RenderingSystem::State(states) => {
                    for pane in states.values() {
                        sprites.insert(pane.sprite.clone());
                    }
                }
            }
        }
    }

    fn lua_registry_name() -> &'static str {
        "Entities"
    }
}

impl Prototype for VelocityCompPrototype {
    type Item = VelocityComp;

    fn create(&self, _: RawId) -> Self::Item {
        VelocityComp {
            velocity: Pos {
                x: self.x,
                y: self.y,
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct VelocityCompPrototype {
    pub x: f32,
    pub y: f32,
}

impl LuaConvertableCar for VelocityCompPrototype {
    fn from_luaagh(table: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::Table(table) = table {
            Ok(VelocityCompPrototype {
                x: table.get::<_, LuaCar<_>>("x")?.0,
                y: table.get::<_, LuaCar<_>>("y")?.0,
            })
        } else {
            Err(mlua::Error::UserDataTypeMismatch)
        }
    }

    fn into_luaagh(self, _: &mlua::Lua) -> mlua::Result<mlua::Value> {
        todo!()
    }
}
