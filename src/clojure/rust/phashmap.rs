use im_rc::hashmap::*;
use im_rc::*;
use std::{any::*, fmt::*, result::*, sync::*};

use intertrait::cast::*;
use intertrait::*;

use super::class;
use super::object;

pub struct SPHashMap {
    inner: HashMap<object::Object, object::Object>,
}

castable_to!(SPHashMap => [sync] object::TObject, Send);

unsafe impl Send for SPHashMap {}

unsafe impl Sync for SPHashMap {}

impl object::TObject for SPHashMap {
    fn get_class<'a>(&'a self) -> &class::SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[object::Object]) -> object::Object {
        todo!()
    }

    fn get(&self, name: usize) -> object::Object {
        todo!()
    }

    fn to_string(&self) -> &str {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &object::Object) -> bool {
        todo!()
    }
}

impl SPHashMap {
    pub fn new() -> SPHashMap {
        SPHashMap {
            inner: HashMap::new(),
        }
    }

    pub fn new_hash(inner: HashMap<object::Object, object::Object>) -> SPHashMap {
        SPHashMap { inner }
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        object::Object::init();
    }
}

static mut INIT: bool = false;
