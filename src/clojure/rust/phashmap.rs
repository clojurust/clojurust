// use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;

type ObjHashMap = im::hashmap::HashMap<Object, Object>;

pub struct SPHashMap {
    inner: ObjHashMap,
}

castable_to!(SPHashMap => [sync] TObject, Send);

unsafe impl Send for SPHashMap {}

unsafe impl Sync for SPHashMap {}

impl TObject for SPHashMap {
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

impl SPHashMap {
    pub fn new() -> SPHashMap {
        SPHashMap {
            inner: ObjHashMap::new(),
        }
    }

    pub fn new_hash(inner: ObjHashMap) -> SPHashMap {
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
