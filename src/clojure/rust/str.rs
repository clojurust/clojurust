//! # Defines Rust dynamic Strings.
//!

use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
use std::{any::*, convert::*, fmt::*, hash::*, result::*, sync::*};

use intertrait::cast::*;
use intertrait::*;

use super::object::*;

pub struct SStr {
    inner: String,
}

trait Str {}

impl Str for SStr {}

/// String -> SStr
impl From<String> for SStr {
    fn from(s: String) -> Self {
        SStr { inner: s }
    }
}

/// &str -> SStr
impl From<&str> for SStr {
    fn from(s: &str) -> Self {
        SStr {
            inner: String::from(s),
        }
    }
}

/// SStr -> Object
/// We call new() because a new object is created.
/// That's why we cannot use the reverse into function
impl From<SStr> for Object {
    fn from(s: SStr) -> Self {
        Object::new::<SStr>(s)
    }
}

/// String -> Object
impl From<String> for Object {
    fn from(s: String) -> Self {
        Object::from(SStr::from(s))
    }
}

/// &str -> Object
impl From<&str> for Object {
    fn from(s: &str) -> Self {
        Object::from(SStr::from(s))
    }
}

castable_to!(SStr => [sync] TObject, Str);

impl TObject for SStr {
    fn get_class(&self) -> Object {
        todo!()
    }

    fn call(&self, name: &Object, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: &Object) -> Object {
        todo!()
    }

    fn to_string(&self) -> Object {
        todo!()
    }

    fn get_hash(&self) -> Object {
        todo!()
    }
}
