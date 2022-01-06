use std::fmt::Display;

use mlua::{Error, Integer, MetaMethod, Number, Table, UserData};

#[derive(Copy, Clone)]
pub struct Tile {
    pub flavour: u8,
}

impl UserData for Tile {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("flavour", |_, this| Ok(this.flavour));
        fields.add_field_method_set("flavour", |_, this, val| {
            this.flavour = val;
            Ok(())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Call, |_, ()| Ok(Tile { flavour: 69 as u8 }));
        methods.add_function("new", |_, table: Table| {
            let i = table.get::<&str, i32>("flavour").map_err(|_| Error::RuntimeError("\"flavour\" parameter missing".parse().unwrap()))? as u8;
            Ok(Tile { flavour: i })
        })
    }
}