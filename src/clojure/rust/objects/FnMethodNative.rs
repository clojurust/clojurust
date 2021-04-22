//! # Anonymous Native (Rust) method with a defined arity
//!
//! This is a Method and so use an object as first argument.
//! The method is linked with the `Object`'s `Class` say their protocols.
//!
//! Method can be with multi-arity according to SFunction.multiarity.
//! If this value is Nil, no multi-arity, else the value is the arity
//! of the multi-arity function, which should be the last one.

use std::{fmt::*};

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

use intertrait::*;
castable_to!(SFnMethodNative => [sync] IObject, FnMethodNative);

init_obj! {
    FnMethodNative {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Class::init();
    }
}

pub struct SFnMethodNative {
    inner: fn(&[Object]) -> ObjResult<Object>
}

impl Debug for SFnMethodNative {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Method native")
    }
}

pub trait FnMethodNative: IObject {
    fn call(&self, args: &[Object]) -> ObjResult<Object>;
}

impl FnMethodNative for SFnMethodNative {
    fn call(&self, args: &[Object]) -> ObjResult<Object> {
        let f = self.inner;
        f(args)
    }
}

impl IObject for SFnMethodNative {
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
