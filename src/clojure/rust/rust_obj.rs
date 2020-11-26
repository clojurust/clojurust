// use std::sync::*;
use lazy_static::lazy_static;
use im::hashmap::*;
use super::object::*;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct RustObj {
    pub objects: RwLock<HashMap<String, Arc<Object>>>,
}

impl RustObj {
    pub fn new() {

    }
    
    pub fn current() -> Arc<RustObj> {
        RUST_OBJ.read().unwrap().clone()
    }

    pub fn make_current(self) {
        *RUST_OBJ.write().unwrap() = Arc::new(self);
    }

    fn update(name: String, obj: Arc<Object>) {
        RustObj{objects: RwLock::new(RustObj::current()
                .objects.write().unwrap()
                .update(name, obj.clone()))}.make_current();
    }
}

pub fn init() {
    RustObj { objects: RwLock::new(HashMap::<String, Arc<Object>>::new()), }.make_current()
}

lazy_static! {
//   static ref RUST_OBJ: Arc<RwLock<RustObj>> = Arc::new(RwLock::new(RustObj::new()));
    pub static ref RUST_OBJ: RwLock<Arc<RustObj>> = RwLock::new(Default::default());
}

