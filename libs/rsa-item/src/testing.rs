use rsa_core::lua::{Lua, LuaResult};
use apollo::{lua_method, lua_field, lua_impl};
use crate::stack::ItemStack;

pub struct Test {
	pub ty: ItemStack,
}

#[lua_impl]
impl Test {
	#[lua_field]
	pub fn get_ty(&mut self) -> LuaResult<&mut ItemStack> {
		Ok(&mut self.ty)
	}

	#[lua_field]
	pub fn set_ty(&mut self, value: &mut ItemStack) -> LuaResult<()> {
		self.ty = value.clone();
		Ok(())
	}

	#[lua_method]
	pub fn to_stack(&self, amount: Option<u32>) -> LuaResult<ItemStack> {
		todo!()
	}

	#[lua_method]
	pub fn thing(&self, number: u32)  -> LuaResult<()> {
		println!("{}", number);
		Ok(())
	}

	#[lua_method]
	pub fn __tostring(&self, lua: &Lua) -> LuaResult<String> {
		todo!()
	}
}
