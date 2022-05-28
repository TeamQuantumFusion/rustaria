use std::cell::UnsafeCell;
use std::sync::Arc;
use type_map::TypeMap;
use crate::blake3::Blake3Hash;
use crate::registry::Registry;
use crate::ty::Prototype;

/// # Carrier has arrived!
/// A carrier of all of the registries and the core hash.
#[derive(Clone)]
pub struct Carrier {
	/// Describes an item and its properties.
	pub(crate) data: Arc<UnsafeCell<CarrierData>>,
}

unsafe impl Send for Carrier {}
unsafe impl Sync for Carrier {}

impl Carrier {
	/// use `get_carrier` from api
	pub(crate) fn new() -> Carrier {
		Carrier {
			data: Arc::new(UnsafeCell::new(CarrierData { registries: Default::default(), hash: Default::default() }))
		}
	}

	fn get_data(&self) -> &CarrierData {
		unsafe { &*(self.data.get() as *const CarrierData) }
	}

	pub fn get_hash(&self) -> Blake3Hash {
		self.get_data().hash
	}

	pub fn get<P: Prototype>(&self) -> &Registry<P> {
		self.get_data()
			.registries
			.get::<Registry<P>>()
			.expect(&*format!(
				"Could not find registry containing {}",
				P::lua_registry_name()
			))
	}
}

pub struct CarrierData {
	pub(crate) registries: TypeMap,
	pub(crate) hash: Blake3Hash,
}


