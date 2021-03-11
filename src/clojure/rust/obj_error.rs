use std::error::*;
use std::fmt;

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

pub struct SObjError {
    msg: String,
}

castable_to!(SObjError => [sync] TObject, ObjError);

pub trait ObjError {}

impl ObjError {}

impl ObjError for SObjError {}

impl Error for SObjError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for SObjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Debug for SObjError {
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

pub fn error<T>(msg: &str) -> Result<&'static T, SCljError> {
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
