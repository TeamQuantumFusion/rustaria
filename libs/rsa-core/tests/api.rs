use std::sync::RwLock;
use mlua::prelude::LuaUserData;
use mlua::{UserDataFields, UserDataMethods};
use rsa_core::api::Api;
use rsa_core::api::carrier::Carrier;
use rsa_core::lua::glue::{Glue, LuaGlue};
use rsa_core::plugin::archive::{Archive, TestAsset};
use rsa_core::plugin::Plugin;
use rsa_core::reload;
use rsa_core::ty::Tag;

pub struct System {
	eggs: u32
}

impl System {
	pub fn egg_inc(&mut self, amount: u32) {
		self.eggs += amount;
	}
}

impl LuaUserData for System {
	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_method_mut("egg_inc", |_, value, args: u32| {
			value.egg_inc(args);
			Ok(())
		})
	}
}

#[test]
pub fn basic() -> rsa_core::error::Result<()> {
	let mut system = System {
		eggs: 1
	};


	rsa_core::initialize().unwrap();
	let mut api = Api::new_test();
	let mut carrier = api.get_carrier();
	api.load_test_plugins(vec![Plugin::new_test(
		"test",
		Archive::new_test(vec![TestAsset::lua(
			"entry", r#"
				-- hello there
				hook:subscribe("rustaria:test", function(system)
					system:egg_inc(1);
				end)
				"#,
		)]),
		&api,
	)]);

	reload!(() => api, carrier);


	assert_eq!(system.eggs, 1);

	Glue::scope(&mut system, |system| {
		api.invoke_hook(&Tag::rsa("test"), move || {
			system
		}).unwrap();
	});

	assert_eq!(system.eggs, 2);
	Ok(())
}

