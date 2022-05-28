use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub type Rcl<T> = Rc<RwLock<T>>;
pub type Arcl<T> = Arc<RwLock<T>>;