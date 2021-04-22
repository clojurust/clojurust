
use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    Thread {
        // clojure::rust::Object::init();
        // clojure::rust::ObjResult::init();
        clojure::rust::IObject::init();
    }
}

pub trait Thread: IObject {

}