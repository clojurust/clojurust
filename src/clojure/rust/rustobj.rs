// use lazy_static::lazy_static;

// use std::sync::{Arc, RwLock};

// use std::borrow::Borrow;

// use cast::CastRef;
// use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;
use super::pvector::*;

pub struct SRustObj {
    pub obj: Object, // SPVector
}

castable_to!(SRustObj => [sync] TObject, RustObj);

pub trait RustObj {
    fn update(&mut self, index: usize, value: &Object) -> Object;

    fn add(&mut self, value: &Object) -> Object;

    fn get(&mut self, index: usize) -> Object;
}

impl RustObj for SRustObj {
    fn update(&mut self, index: usize, value: &Object) -> Object {
        let v = self.obj.inn_mut::<SPVector>().update(index, value.clone());
        Object::new::<SPVector>(v)
    }

    fn add(&mut self, value: &Object) -> Object {
        let v = self.obj.inn_mut::<SPVector>();
        let o = v.add(value.clone());
        Object::new::<SPVector>(o)
    }

    fn get(&mut self, index: usize) -> Object {
        PVector::get(self.obj.inn_mut::<SPVector>(), index).clone()
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
    fn get_class<'a>(&self) -> &'a SClass {
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
