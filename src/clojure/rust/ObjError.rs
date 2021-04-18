//! # Standard `Error`s for the library
//!
//! A lot TODO

use std::fmt;
use std::io;

// use intertrait::cast::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
}

castable_to!(SObjError => [sync] TObject, ObjError);

init_obj! {
    ObjError {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}

pub type ObjResult<T> = std::result::Result<T, SObjError>;

#[derive(Debug)]
pub enum ErrorType {
    BadCast,
    NotFound,
    Arity,
    Error
}

#[derive(Debug)]
/// Standard error for the library
pub struct SObjError {
    /// Error message with format
    msg: String,
    err: ErrorType
}

/// `Protocol` ObjError
pub trait ObjError: CastFromSync {}

impl ObjError {}

impl ObjError for SObjError {}

impl From<io::Error> for SObjError {
    fn from(_: io::Error) -> Self {
        SObjError {
            msg: String::from("Error"),
            err: ErrorType::Error,
            
        }
    }
}

impl fmt::Display for SObjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^ObjError {:?}", self)
    }
}

impl TObject for SObjError {
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

pub fn err<T>(msg: &str) -> ObjResult<T> {
    Err(SObjError {
        msg: String::from(msg),
        err: ErrorType::Error
    })
}

pub fn err_cast<T>(from: &Object, to: &str) -> ObjResult<T> {
    Err(SObjError {
        msg: format!("Cannot cast {:?} to {:?}", from, to),
        err: ErrorType::BadCast
    })
}

pub fn err_not_found<T>(what: &Object, into: &Object) -> ObjResult<T> {
    Err(SObjError {
        msg: format!("Not found {:?} in {:?}", what, into),
        err: ErrorType::NotFound
    })
}

pub fn err_arity<T>(arity: usize, obj: &Object) -> ObjResult<T> {
    Err(SObjError {
        msg: format!("Bad Arity {:?} on {:?}", arity, obj),
        err: ErrorType::Arity
    })
}

#[test]
fn error_test() {
}
