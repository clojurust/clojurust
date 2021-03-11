// use lazy_static::__Deref;
// use std::{any::*, result::*, sync::*};
// use std::{fmt::*, hash::*};

// use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;

pub type SObjVector = im::vector::Vector<Object>;

castable_to!(SObjVector => [sync] TObject, ObjVector);

impl TObject for SObjVector {
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

pub trait ObjVector: CastFromSync {}

impl ObjVector {
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

impl ObjVector for SObjVector {}

static mut INIT: bool = false;
