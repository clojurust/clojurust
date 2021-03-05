// use lazy_static::lazy_static;

// use std::sync::{Arc, RwLock};

// use intertrait::cast::*;
use intertrait::*;

use super::object::*;
use super::pvector::*;

pub struct SRustObj {
    pub obj: Object, // SPVector
}

castable_to!(SRustObj => [sync] TObject, RustObj);

pub trait RustObj {
    fn update(&self, index: usize, value: &Object) -> Object;

    fn add(&self, value: &Object) -> Object;

    fn get(&self, index: usize) -> Object;
}

impl RustObj for SRustObj {
    fn update(&self, index: usize, value: &Object) -> Object {
        let o = self.obj.inn::<SPVector>().update(index, value.clone());
        Object::new::<SPVector>(o)
    }

    fn add(&self, value: &Object) -> Object {
        let o = self.obj.inn::<SPVector>().add(value.clone());
        Object::new::<SPVector>(o)
    }

    fn get(&self, index: usize) -> Object {
        PVector::get((&self.obj).inn::<SPVector>(), index).clone()
    }
}

impl SRustObj {
    pub fn new() -> SRustObj {
        SRustObj {
            obj: Object::new::<SPVector>(SPVector::new()),
        }
    }

    fn new_vect(new: Object) -> Object {
        Object::new::<SRustObj>(SRustObj { obj: new })
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        Object::init();
        SPVector::init();
    }
}

impl TObject for SRustObj {
    fn get_class(&self) -> &super::class::SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: usize) -> Object {
        todo!()
    }

    fn to_string(&self) -> &str {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

static mut INIT: bool = false;

#[test]
fn test_rust_obj() {}
