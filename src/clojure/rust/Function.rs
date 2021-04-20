//! # Anonymous Function with multi-arity
//!
//! This is a map of

// use lazy_static::{__Deref, lazy_static};
use std::{fmt::*, sync::*};

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::rust::Class;
}

castable_to!(SFunction => [sync] IObject, Function);

init_obj! {
    Function {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Class::init();
    }
}

#[derive(Debug)]
pub struct SFunction {
    /// index of full name: ns + class/protocol + name
    pub full_name: usize,
    /// Mark optional arity of multi-arity function.
    pub multiary: Option<usize>,
    /// ObjHashMap arity `usize` ->
    pub func: Object,
}

unsafe impl Send for SFunction {}

unsafe impl Sync for SFunction {}

pub trait Function: IObject + CastFromSync {
    fn call(&self, args: &Object) -> Object;

    fn get(&self, arity: usize) -> Object;
}

impl Function for SFunction {
    fn get(&self, arity: usize) -> Object {
        // let mut index = arity;
        // match self.multiary {
        //     Some(max) => {
        //         if arity > max {
        //             index = max;
        //         }
        //         let implem = Object::cast::<SFunction>(&self.func);
        //         let funcs = implem.get(index);
        //         let a = Object::cast::<SFnMethodNative>(&implem);
        //         let fn_nat = Object::cast::<SFnMethodNative>(a);
        //         Object::null()
        //     },
        //     // If no max => no implementation
        //     None => todo!(),
        // }
        todo!()
    }

    fn call(&self, args: &Object) -> Object {
        Object::null()
    }
}

impl Display for SFunction {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "function {}", self.full_name)
    }
}

impl IObject for SFunction {
    fn get_class<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

use crate::new_obj;

impl SFunction {
    pub fn new(full_name: usize, multiary: Option<usize>, func: Object) -> Object {
        new_obj!(SFunction { full_name, multiary, func })
    }
}

impl Default for SFunction {
    fn default() -> Self {
        todo!()
    }
}
