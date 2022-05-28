use crate::blake3::Blake3Hash;
use crate::registry::Registry;
use crate::ty::Prototype;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};

use std::sync::Arc;
use type_map::concurrent::TypeMap;

pub type RegistryLock<'a, P> = MappedRwLockReadGuard<'a, Registry<P>>;

/// # Carrier has arrived!
/// A carrier of all of the registries and the core hash.
#[derive(Clone)]
pub struct Carrier {
	/// Describes an item and its properties.
	pub(crate) data: Arc<RwLock<CarrierData>>,
}

impl Carrier {
	/// use `get_carrier` from api
	pub(crate) fn new() -> Carrier {
		Carrier {
			data: Arc::new(RwLock::new(CarrierData {
				registries: Default::default(),
				hash: Default::default(),
			})),
		}
	}

	pub fn get_hash(&self) -> Blake3Hash {
		self.data.read().hash
	}

	pub fn get<P: Prototype>(&self) -> RegistryLock<P> {
		RwLockReadGuard::map(self.data.read(), |data| {
			data.registries.get::<Registry<P>>().expect(&*format!(
				"Could not find registry containing {}",
				P::lua_registry_name()
			))
		})
	}
}

pub struct CarrierData {
	pub(crate) registries: TypeMap,
	pub(crate) hash: Blake3Hash,
}
