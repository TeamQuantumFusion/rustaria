use eyre::Result;

use rustaria_api::{Api, Carrier};

#[macro_use]
pub mod prototype;

pub mod ty;
#[cfg(feature = "client")]
pub mod rendering;

pub fn reload(api: &mut Api, carrier: &mut Carrier) -> Result<()> {
    let mut reload = api.reload(carrier);
    prototypes!({ reload.add_reload_registry::<P>()? });
    reload.reload();
    prototypes!({ reload.add_apply_registry::<P>()? });
    reload.apply();
    Ok(())
}