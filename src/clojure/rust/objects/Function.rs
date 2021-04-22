//! # Anonymous Function with multi-arity
//!
//! This is a map of

// use lazy_static::{__Deref, lazy_static};
use std::sync::*;

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

use intertrait::*;
castable_to!(SFunction => [sync] IObject, Function);

init_obj! {
    Function {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Class::init();
    }
}

pub struct SFunction {
    /// index of full name: ns + class/protocol + name
    pub full_name: usize,
    /// Mark optional arity of multi-arity function.
    pub multiary: Option<usize>,
    /// PersistentHashMap arity `usize` ->
    pub func: Object,
}

unsafe impl Send for SFunction {}

unsafe impl Sync for SFunction {}

pub trait Function: IObject + CastFromSync {
    fn call(&self, args: &Object) -> ObjResult<Object>;

    fn get(&self, arity: usize) -> ObjResult<Object>;
}

impl Function for SFunction {
    fn get(&self, arity: usize) -> ObjResult<Object> {
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

    fn call(&self, args: &Object) -> ObjResult<Object> {
        todo!()
    }
}

impl IObject for SFunction {
    fn getClass<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn hashCode(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }

    fn toString(&self) -> usize {
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
