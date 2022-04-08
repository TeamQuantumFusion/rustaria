use std::ops::Deref;

use rustaria_api::{ApiHandler};
use rustaria_util::Result;

#[macro_use]
pub mod prototype;
pub mod ty;

pub struct Api {
    instance: ApiHandler,
}

impl Api {
    pub fn reload(&mut self) -> Result<()> {
        let mut reload = self.instance.reload();
        prototypes!({ reload.register_builder::<P>()? });
        reload.reload()?;
        prototypes!({ reload.compile_builder::<P>()? });
        reload.apply();
        Ok(())
    }
}

impl Deref for Api {
    type Target = ApiHandler;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name;

    #[test]
    fn test() {
        prototypes!({ println!("{}", type_name::<P>()) });
    }
}
