// use std::sync::*;
use lazy_static::lazy_static;

// use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;
use super::rustobj::*;
use super::unique::*;

pub struct SGlobals {
    pub id: Object,  // SUnique
    pub obj: Object, // SRustObj
}

castable_to!(SGlobals => [sync] TObject, Globals);

pub trait Globals {
    fn update_object(&mut self, index: usize, value: &Object) -> Object;

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
    fn update_object(&mut self, index: usize, value: &Object) -> Object {
        let v = self.obj.inn_mut::<SRustObj>().update(index, &value.clone());
        Object::new::<SGlobals>(SGlobals {
            id: self.id.clone(),
            obj: v,
        })
    }

    fn get_obj_by_id(&self, index: usize) -> Object {
        self.obj.get(index)
    }
}

impl TObject for SGlobals {
    fn get_class<'a>(&self) -> &'a SClass {
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
