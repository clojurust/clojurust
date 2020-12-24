// use std::sync::*;
use lazy_static::lazy_static;
use im::vector::*;
use std::sync::{Arc, RwLock};

pub use crate::clojure;
use clojure::core::object::*;


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

    pub fn get(index: usize) -> Object {
        match RustObj::current().objects.get(index)
        {
            Some(obj) => {obj.clone()}
            None => {RustObj::null()}
        }
    }

    pub fn null() -> Object {
        RustObj::current().objects.get(0).unwrap().clone()
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
