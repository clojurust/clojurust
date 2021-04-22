//! # Defines library's dynamic Strings.
//!

use std::{convert::*, fmt::*};
use std::sync::*;

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

use intertrait::*;
castable_to!(string => [sync] IObject);

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

    fn toString(&self) -> String {
        todo!()
    }
}
