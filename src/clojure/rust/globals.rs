// use std::sync::*;
use im_rc::vector::*;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

use intertrait::cast::*;
use intertrait::*;

use super::object::*;
use super::pvector::*;
use super::rustobj::*;
use super::unique::*;

pub struct SGlobals {
    pub id: Object,
    pub obj: Object,
}

castable_to!(SGlobals => [sync] TObject, Globals);

pub trait Globals {
    fn update_object(&self, index: usize, value: &Object) -> Object;

    fn get_obj_by_id(&self, index: usize) -> Object;
}

impl SGlobals {
    pub fn new() -> SGlobals {
        SGlobals {
            id: Object::new::<SUnique>(SUnique::new()),
            obj: Object::new::<SRustObj>(SRustObj::new()),
        }
    }
}

impl Globals for SGlobals {
    fn update_object(&self, index: usize, value: &Object) -> Object {
        Object::new::<SGlobals>(SGlobals {
            id: self.id.clone(),
            obj: Object::new::<SPVector>(self.obj.update(index, value)),
        })
    }

    fn get_obj_by_id(&self, index: usize) -> Object {
        self.obj.get(index)
    }
}

impl TObject for SGlobals {
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

pub fn init() {}

lazy_static! {
    pub static ref RUSTOBJ: SGlobals = SGlobals::new();
}

#[test]
fn test_rust_obj() {}
