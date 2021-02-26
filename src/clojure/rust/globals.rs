// use std::sync::*;
use im::vector::*;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

use super::object::*;
use super::pvector::*;
use super::rustobj::*;
use super::unique::*;

pub struct Globals {
    pub id: Arc<Unique>,
    pub obj: Arc<RustObj>,
}

impl Globals {
    pub fn new() -> Globals {
        Globals {
            id: Arc::new(Unique::new()),
            obj: Arc::new(RustObj::new()),
        }
    }

    fn update_object(&self, index: usize, value: &SObject) -> Globals {
        Globals {
            id: self.id.clone(),
            obj: Arc::new(self.obj.update(index, value)),
        }
    }

    pub fn get_obj_by_id(&self, index: usize) -> SObject {
        self.obj.get(index)
    }
}

pub fn init() {}

lazy_static! {
    pub static ref RUSTOBJ: Globals = Globals::new();
}

#[test]
fn test_rust_obj() {}
