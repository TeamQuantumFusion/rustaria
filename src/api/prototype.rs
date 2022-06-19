use crate::{api::luna::table::LunaTable};

pub trait Prototype
where
	Self: Sized + 'static + Send,
{
	type Output;
	fn get_name() -> &'static str;
	fn from_lua(table: LunaTable) -> eyre::Result<Self>;
}
