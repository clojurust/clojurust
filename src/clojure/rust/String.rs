//! # Defines library's dynamic Strings.
//!

use std::{convert::*, fmt::*};
use std::sync::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::Class;
}

use intertrait::*;
castable_to!(string => [sync] IObject);

init_obj! {
    Stri {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::Class::init();
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct string {
    pub inner: String,
}

impl string {}

/// string -> string
impl From<String> for Object {
    fn from(s: String) -> Self {
        new_obj!(string { inner: s })
    }
}

/// &str -> string
impl From<&str> for Object {
    fn from(s: &str) -> Self {
        new_obj!(string { inner: String::from(s)})
    }
}

impl IObject for string {
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
