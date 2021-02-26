pub use im::hashmap::*;
use im::*;
use intertrait::cast::*;
use intertrait::*;
use std::{any::*, fmt::*, result::*, sync::*};

use super::class::*;
use super::object::*;

pub struct PHashMap {
    inner: HashMap<SObject, SObject>,
}

castable_to!(PHashMap => Object);

impl Object for PHashMap {
    fn get_class<'a>(&'a self) -> &'a Class {
        todo!()
    }

    fn call(&self, name: &str, args: &[SObject]) -> SObject {
        todo!()
    }

    fn get(&self, name: &str) -> SObject {
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
    pub fn new(inner: HashMap<SObject, SObject>) -> PHashMap {
        PHashMap { inner }
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        SObject::init();
    }
}

static mut INIT: bool = false;
