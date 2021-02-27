use im::hashmap::*;
use im::*;
use intertrait::cast::*;
use intertrait::*;
use std::{any::*, fmt::*, result::*, sync::*};

use super::class::*;
use super::object::*;

pub struct PHashMap {
    inner: HashMap<Object, Object>,
}

castable_to!(PHashMap => TObject);

impl TObject for PHashMap {
    fn get_class<'a>(&'a self) -> Object {
        todo!()
    }

    fn call(&self, name: &str, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: &str) -> Object {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }
}

impl PHashMap {
    pub fn new() -> PHashMap {
        PHashMap {
            inner: HashMap::new(),
        }
    }

    pub fn new_hash(inner: HashMap<Object, Object>) -> PHashMap {
        PHashMap { inner }
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        Object::init();
    }
}

static mut INIT: bool = false;
