//! # Defines library's dynamic Strings.
//!

use std::{convert::*, fmt::*};
use std::sync::*;

// use intertrait::cast::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
}

castable_to!(SStri => [sync] TObject, Stri);

init_obj! {
    Stri {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}

#[derive(Debug)]
pub struct SStri {
    pub inner: String,
}

pub trait Stri: CastFromSync {}

impl Stri {}

impl Stri for SStri {}

/// String -> SStr
impl From<String> for SStri {
    fn from(s: String) -> Self {
        SStri { inner: s }
    }
}

/// &str -> SStr
impl From<&str> for SStri {
    fn from(s: &str) -> Self {
        SStri {
            inner: String::from(s),
        }
    }
}

/// SStr -> Object
/// We call new() because a new object is created.
/// That's why we cannot use the reverse into function
impl From<SStri> for Object {
    fn from(s: SStri) -> Self {
        new_obj!(s)
    }
}

/// String -> Object
impl From<String> for Object {
    fn from(s: String) -> Self {
        Object::from(SStri::from(s))
    }
}

/// &str -> Object
impl From<&str> for Object {
    fn from(s: &str) -> Self {
        Object::from(SStri::from(s))
    }
}

castable_to!(SStri => [sync] TObject, Stri);

impl Display for SStri {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\"{:?}\"", self.inner)
    }
}

impl TObject for SStri {
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
