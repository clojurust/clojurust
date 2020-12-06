// use std::sync::*;
use lazy_static::lazy_static;
use im::vector::*;
use super::object::*;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct RustObj {
    pub objects: Vector<Object>,
}

impl RustObj {
    pub fn current() -> Arc<RustObj> {
        RUST_OBJ.read().unwrap().clone()
    }

    pub fn make_current(self) {
        *RUST_OBJ.write().unwrap() = Arc::new(self);
    }

    pub fn update(name: usize, obj: Object) {
        RustObj{objects: RustObj::current()
                .objects
                .update(name, obj.clone())}.make_current();
    }

    pub fn get(name: usize) -> Option<Object> {
        match RustObj::current().objects.get(name)
        {
            Some(obj) => {Some(obj.clone())}
            None => {None}
        }
    }
}

pub fn init() {
    RustObj { objects: Vector::<Object>::new(), }.make_current()
}

lazy_static! {
//   static ref RUST_OBJ: Arc<RwLock<RustObj>> = Arc::new(RwLock::new(RustObj::new()));
    pub static ref RUST_OBJ: RwLock<Arc<RustObj>> = RwLock::new(Default::default());
}

#[test]
fn test_rust_obj() {
    

}
