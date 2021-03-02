use im_rc::hashmap::*;
use im_rc::*;
use intertrait::cast::*;
use intertrait::*;
use std::{any::*, fmt::*, result::*, sync::*};

use super::class::*;
use super::object::*;

pub struct SPHashMap {
    inner: HashMap<Object, Object>,
}

castable_to!(SPHashMap => [sync] TObject, Send);

unsafe impl Send for SPHashMap {}

unsafe impl Sync for SPHashMap {}

impl TObject for SPHashMap {
    fn get_class<'a>(&'a self) -> &SClass {
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

impl SPHashMap {
    pub fn new() -> SPHashMap {
        SPHashMap {
            inner: HashMap::new(),
        }
    }

    pub fn new_hash(inner: HashMap<Object, Object>) -> SPHashMap {
        SPHashMap { inner }
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
