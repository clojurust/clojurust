//! # Defines Rust dynamic Strings.
//!

use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
use std::{any::*, convert::*, fmt, hash::*, result::*, sync::*};

use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::error::*;
use super::object::*;

pub struct SStri {
    inner: String,
}

pub trait Stri {}

impl<'a> TryInto<&'a SStri> for &'a Object {
    type Error = SCljError;

    fn try_into(self) -> Result<&'a SStri, Self::Error> {
        let o = self.clone();
        match o.inner {
            None => error::<SStri>("Convert nil to string"),
            Some(o) => {
                let a = o.as_ref().cast::<SStri>();
                match a {
                    None => error::<SStri>("Convert nil to string"),
                    Some(o) => Ok(o),
                }
            }
        }
    }
}

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
        Object::new::<SStri>(s)
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

impl TObject for SStri {
    fn get_class(&self) -> &SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: usize) -> Object {
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
