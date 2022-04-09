use mlua::prelude::{LuaMetaMethod, LuaUserData, LuaUserDataFields, LuaUserDataMethods};
use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

// This is lua input (or rust) that gets converted to id,
// by the registry map.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Tag {
    inner: String,
    colon_index: usize,
}
impl Tag {
    pub fn from_string(s: String) -> Result<Self, ParseTagError> {
        let colon_index = s.find(':').ok_or(ParseTagError::NotColonSeparated)?;
        Ok(Self {
            inner: s,
            colon_index,
        })
    }
    pub fn plugin_id(&self) -> &str {
        &self.inner[..self.colon_index]
    }
    pub fn name(&self) -> &str {
        &self.inner[self.colon_index + 1..]
    }
}

impl FromStr for Tag {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_index = s.find(':').ok_or(ParseTagError::NotColonSeparated)?;
        Ok(Self {
            inner: s.to_string(),
            colon_index,
        })
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl LuaUserData for Tag {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(f: &mut F) {
        f.add_field_method_get("plugin_id", |_, t| Ok(t.plugin_id().to_owned()));
        f.add_field_method_get("name", |_, t| Ok(t.name().to_owned()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(m: &mut M) {
        m.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(format!("{this}"))
        });
    }
}

#[derive(Clone, Copy, Debug, Error)]
pub enum ParseTagError {
    #[error("Tag is not encoded as a colon-separated string")]
    NotColonSeparated,
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de;

        struct TagVisitor;
        impl<'de> de::Visitor<'de> for TagVisitor {
            type Value = Tag;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a colon-separated string representing a registry tag")
            }
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Tag::from_str(v).map_err(de::Error::custom)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Tag::from_str(v).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_str(TagVisitor)
    }
}
