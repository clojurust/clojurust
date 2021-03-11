// use lazy_static::__Deref;
// use std::{any::*, result::*, sync::*};
// use std::{fmt::*, hash::*};

// use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;

pub type SObjHashSet = im::hashset::HashSet<Object>;

castable_to!(SObjHashSet => [sync] TObject, ObjHashSet);

impl TObject for SObjHashSet {
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

pub trait ObjHashSet: CastFromSync {}

impl ObjHashSet for SObjHashSet {}

impl ObjHashSet {
    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        println!("Class::init");

        // Insures all is initialized
        Object::init();
        SClass::init();
        // let c = Keywords::get("clojure.rust.object/Objects");
    }
}

static mut INIT: bool = false;
