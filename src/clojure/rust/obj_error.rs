//! # Standard `Error`s for the library
//!
//! A lot TODO

use std::fmt;

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

pub struct SObjError {
    msg: String,
    previous: Object,
}

castable_to!(SObjError => [sync] TObject, ObjError);

pub trait ObjError {}

impl ObjError {}

impl ObjError for SObjError {}

impl ObjError for SObjError {}

impl fmt::Display for SObjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

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

pub fn error<T>(msg: &str) -> Result<&'static T, SObjError> {
    Err(SObjError {
        msg: String::from(msg),
    })
}

pub unsafe fn init() {
    // only execute one time
    if INIT {
        return;
    }
    INIT = true;

    println!("Error::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
}

static mut INIT: bool = false;
