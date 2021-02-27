// use std::sync::*;
use im::vector::*;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

use super::object::*;
use super::pvector::*;

pub struct RustObj {
    pub obj: PVector,
}

impl RustObj {
    pub fn new() -> RustObj {
        RustObj {
            obj: PVector::new(),
        }
    }

    fn new_vect(new: PVector) -> RustObj {
        RustObj { obj: new }
    }

    pub fn update(&self, index: usize, value: &Object) -> RustObj {
        RustObj {
            obj: {
                if index == self.obj.len() {
                    self.obj.add(value)
                } else {
                    self.obj.update(index, value)
                }
            },
        }
    }

    pub fn get(&self, index: usize) -> Object {
        self.obj.get(index)
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        Object::init();
        PVector::init();
    }
}

static mut INIT: bool = false;

#[test]
fn test_rust_obj() {}
