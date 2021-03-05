// use std::sync::*;
use lazy_static::lazy_static;

// use intertrait::cast::*;
use intertrait::*;

use super::object::*;
use super::rustobj::*;
use super::unique::*;

pub struct SGlobals {
    pub id: Object,  // SUnique
    pub obj: Object, // SRustObj
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
        let new_obj = self.clone();
        Object::new::<SGlobals>(SGlobals {
            id: new_obj.id.clone(),
            obj: {
                let a = new_obj.obj.clone();
                let b = a.inn::<SRustObj>();
                let c = b.update(index, value);
                c
            },
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
