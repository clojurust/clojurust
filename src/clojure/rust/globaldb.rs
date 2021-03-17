//! Global information on library data structures

use crate::use_obj;

use_obj! {
    clojure::rust::class;
    clojure::rust::object;
    clojure::rust::globals;
}

castable_to!(SGlobalDB => [sync] TObject, GlobalDB);

init_obj! {
    GlobalDB {
        clojure::rust::object::init();
        clojure::rust::class::init();
        clojure::rust::globals::init();
    }
}

pub struct SGlobalDB {
    rust_obj: SGlobals,
    names: SGlobals,
}

pub trait GlobalDB {}

impl GlobalDB {}

impl GlobalDB for SGlobalDB {}

impl TObject for SGlobalDB {
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
