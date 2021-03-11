// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

type SObjHashMap = im::hashmap::HashMap<Object, Object>;

castable_to!(SObjHashMap => [sync] TObject, ObjHashMap);

pub trait ObjHashMap {}

impl TObject for SObjHashMap {
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

impl ObjHashMap for SObjHashMap {}

impl ObjHashMap {}

pub unsafe fn init() {
    // only execute one time
    if INIT {
        return;
    }
    INIT = true;

    println!("Nil::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
}

static mut INIT: bool = false;
