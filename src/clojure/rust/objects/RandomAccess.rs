

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    RandomAccess {
        // clojure::rust::Object::init();
        clojure::rust::IObject::init();
        // clojure::rust::Class::init();
    }
}

pub trait RandomAccess: IObject {}