//! # Standard `Error`s for the library
//!
//! A lot TODO

use std::fmt;

// use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SObjError => [sync] TObject, ObjError);

init_obj! {
    ObjError {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

pub type Result<T> = std::result::Result<T, SObjError>;

/// Standard error for the library
pub struct SObjError {
    /// Error message with format
    msg: String,
}

/// `Protocol` ObjError
pub trait ObjError: CastFromSync {}

impl ObjError {}

impl ObjError for SObjError {}

// impl fmt::Display for SObjError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

impl fmt::Debug for SObjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl TObject for SObjError {
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

pub fn error<T>(msg: &str) -> Result<T> {
    Err(SObjError {
        msg: String::from(msg),
    })
}
