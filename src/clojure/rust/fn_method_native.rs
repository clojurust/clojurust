//! # Anonymous Native (Rust) method with a defined arity
//!
//! This is a Method and so use an object as first argument.
//! The method is linked with the `Object`'s `Class` say their protocols.
//!
//! Method can be with multi-arity according to SFunction.multiarity.
//! If this value is Nil, no multi-arity, else the value is the arity
//! of the multi-arity function, which should be the last one.

use std::sync::Arc;

// use intertrait::cast::*;

/// include and init needed `Rust` `Objects` for `clojure::lang`
use crate::use_obj;
use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SFnMethodNative => [sync] TObject, FnMethodNative);

init_obj! {
    FnMethodNative {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

pub struct SFnMethodNative {
    inner: fn(args: &[Object]) -> Object,
}

pub trait FnMethodNative: CastFromSync {
    fn call(&self, args: &[Object]) -> Object;
}

impl SFnMethodNative {
    fn new(function: fn(args: &[Object]) -> Object) -> Object {
        Object::new(Arc::new(SFnMethodNative { inner: function }))
    }
}

impl FnMethodNative for SFnMethodNative {
    fn call(&self, args: &[Object]) -> Object {
        let f = self.inner;
        f(args)
    }
}

impl TObject for SFnMethodNative {
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
