#[macro_use]
pub mod prototype;

pub mod ty;
#[cfg(feature = "client")]
pub mod rendering;

// #[derive(Clone)]
// pub struct ApiHandler {
//     instance: Arc<RwLock<ApiHandler>>,
// }
// 
// impl ApiHandler {
//     pub fn new(lua: &Lua) -> ApiHandler {
//         ApiHandler {
//             instance: Arc::new(RwLock::new(ApiHandler::new(lua).unwrap())),
//         }
//     }
// 
//     pub fn reload(&mut self, lua: &Lua) -> Result<()> {
//         let mut write = self.instance.write().unwrap();
//         let mut reload = write.reload();
//         prototypes!({ reload.register_builder::<P>(lua)? });
//         reload.reload(lua)?;
//        prototypes!({ reload.compile_builder::<P>(lua)? });
//         reload.apply();
//         Ok(())
//     }
// 
//     pub fn instance(&self) -> RwLockReadGuard<'_, ApiHandler> {
//         self.instance.read().unwrap()
//     }
// }
// 
// #[cfg(test)]
// mod tests {
// 
//     #[test]
//     fn test() {
//     }
// }
