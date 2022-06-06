use std::fmt::{Debug, Formatter};
use rsa_core::api::lua::{FromLua, Lua, LuaError, LuaResult, LuaValue};
use rsa_core::error::Context;
use rsa_core::registry::Registry;
use rsa_core::ty::{Prototype, RawId, Tag};

#[derive(Clone, Debug)]
pub struct AudioPrototype {
	spacial: bool,
	asset: Tag
}

pub struct Audio {
	id: RawId
}

impl FromLua for AudioPrototype {
	fn from_lua(lua_value: LuaValue, lua: &Lua) -> LuaResult<Self> {
		if let LuaValue::Table(table) = lua_value {
			Ok(AudioPrototype {
				spacial: table.get("spacial").unwrap_or(false),
				asset: table.get("asset").wrap_err("Asset is a required field").map_err(|err| LuaError::external(err))?
			})
		} else {
			Err(LuaError::external("wrong type"))
		}
	}
}

impl Prototype for AudioPrototype {
	type Item = Audio;

	fn create(&self, id: RawId) -> Self::Item {
		Audio {
			id
		}
	}

	fn lua_registry_name() -> &'static str {
		"audio"
	}
}

pub struct AudioSystem {
	audio: Registry<AudioPrototype>
}


#[cfg(test)]
mod tests {
	use std::fs;
	use std::io::Cursor;
	use std::time::Duration;
	use rodio::Source;

	#[test]
	pub fn run() {
		let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

		//let sink = rodio::SpatialSink::try_new(
		//	&handle,
		//	[0.0, 0.0, 0.0],
		//	[1.0, 0.0, 0.0],
		//	[-1.0, 0.0, 0.0],
		//)
		//	.unwrap();

		let sink = rodio::Sink::try_new(&handle).unwrap();

		let vec = fs::read("./audio.ogg").unwrap();


		let source = rodio::Decoder::new(Cursor::new(vec)).unwrap().buffered();
		let source = source.convert_samples::<f32>();
		let source = source.low_pass(100);

		sink.append(source);

		sink.sleep_until_end();
	}
}
