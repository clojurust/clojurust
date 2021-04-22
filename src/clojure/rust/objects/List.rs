

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    List {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Collection::init();
    }
}

pub trait List: IObject + Collection {
}