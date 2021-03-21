//! # Anonymous Function with multi-arity
//!
//! This is a map of

// use lazy_static::{__Deref, lazy_static};
use std::sync::*;

use crate::use_obj;

use_obj! {
    clojure::rust::class;
    clojure::rust::fn_method_native;
    clojure::rust::object;
}

castable_to!(SFunction => [sync] TObject, Function);

init_obj! {
    Function {
        clojure::rust::object::init();
        clojure::rust::class::init();
        clojure::rust::fn_method_native::init();
    }
}

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

trait Function: CastFromSync {
    fn call(&self, args: &Object) -> Object;

    fn get(&self, arity: usize) -> Object;
}

impl Function for SFunction {
    fn get(&self, arity: usize) -> Object {
        let mut index = arity;
        match self.multiary {
            Some(max) => {
                if arity > max {
                    index = max;
                }
                let implem = Object::inn::<SFunction>(&self.func).get(index);
                let a = Object::inn::<SFnMethodNative>(&implem);
                let fn_nat = Object::cast::<SFnMethodNative>(a);
            }
            // If no max => no implementation
            None => todo!(),
        }
    }

    fn call(&self, args: &Object) -> Object {
        Object::null()
    }
}

impl TObject for SFunction {
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

impl SFunction {
    pub fn new(multiary: Option<usize>, func: Object) -> Object {
        Object::new(Arc::new(SFunction { multiary, func }))
    }
}

impl Default for SFunction {
    fn default() -> Self {
        todo!()
    }
}
