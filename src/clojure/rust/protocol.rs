//! # Protocol module
//! protocol is the Clojure equivalent of Rust traits.

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

struct SProtocol {
    inner: Object, // SObjHashSet
}

castable_to!(SProtocol => [sync] TObject, Protocol);

impl TObject for SProtocol {
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

pub trait Protocol {}

impl Protocol {}

impl Protocol for SProtocol {}

impl SProtocol {}

pub unsafe fn init() {
    // only execute one time
    if INIT {
        return;
    }

    INIT = true;

    println!("Protocol::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
    clojure::rust::obj_hashset::init();
}

static mut INIT: bool = false;
