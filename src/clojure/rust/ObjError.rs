//! # Standard `Error`s for the library
//!
//! A lot TODO

use std::fmt;

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
    BadCast{
        from: Object, 
        to: Object
    },
    NotFound{
        what: Object, 
        into: Object
    },
    Arity{
        nr: usize
    },
    Compiler{
        data: Object, 
        err_column: usize, 
        err_line: usize, 
        err_ns: String, 
        err_phase: Object
    },
    EdnReader{

    },
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

pub fn error<T>(msg: &str, err: ErrorType) -> ObjResult<T> {
    Err(SObjError {
        msg: String::from(msg),
        err
    })
}

#[test]
fn error_test() {
    let a = error::<String>("test error", ErrorType::Error);
    match a {
        Ok(b) => {
            println!("{:?}", b);
        },
        Err(c) => {
            println!("{:?}", c);
        }
    }
}
