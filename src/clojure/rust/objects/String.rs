//! # Defines library's dynamic Strings.

use std::convert::*;
use std::sync::*;

use clojure::rust::*;
// use clojure::lang::*;
use intertrait::*;

use crate::*;
castable_to!(string => [sync] IObject);

#[allow(non_camel_case_types)]
pub struct string {
    pub inner: String,
}

impl string {
    fn getClass<'a>(&self) -> &'a SClass { todo!() }

    fn hashCode(&self) -> usize { todo!() }

    fn equals(
        &self,
        other: &Object,
    ) -> bool {
        todo!()
    }

    fn toString(&self) -> String { todo!() }
}

impl IObject for string {
    fn getClass<'a>(&self) -> &'a SClass { self.getClass() }

    fn hashCode(&self) -> usize { self.hashCode() }

    fn equals(
        &self,
        other: &Object,
    ) -> bool {
        self.equals(other)
    }

    fn toString(&self) -> String { self.toString() }
}

/// string -> string
impl From<String> for Object {
    fn from(s: String) -> Self {
        new_obj!(string {
            inner: s,
        })
    }
}

/// &str -> string
impl From<&str> for Object {
    fn from(s: &str) -> Self {
        new_obj!(string {
            inner: String::from(s),
        })
    }
}
