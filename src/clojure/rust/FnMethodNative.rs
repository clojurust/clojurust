//! # Anonymous Native (Rust) method with a defined arity
//!
//! This is a Method and so use an object as first argument.
//! The method is linked with the `Object`'s `Class` say their protocols.
//!
//! Method can be with multi-arity according to SFunction.multiarity.
//! If this value is Nil, no multi-arity, else the value is the arity
//! of the multi-arity function, which should be the last one.

use std::{fmt::*};

// use intertrait::cast::*;

/// include and init needed `Rust` `Objects` for `clojure::lang`
use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
}

castable_to!(SFnMethodNative => [sync] TObject, FnMethodNative);

init_obj! {
    FnMethodNative {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}

pub struct SFnMethodNative {
    inner: fn(&[Object]) -> Object
}

impl Debug for SFnMethodNative {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Method native")
    }
}

pub trait FnMethodNative {
    fn call(&self, args: &[Object]) -> Object;
}

impl FnMethodNative for SFnMethodNative {
    fn call(&self, args: &[Object]) -> Object {
        let f = self.inner;
        f(args)
    }
}

impl Display for SFnMethodNative {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Method native")
    }
}

impl TObject for SFnMethodNative {
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
