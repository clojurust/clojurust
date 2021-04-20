
use crate::*;

use_obj! {
    // clojure::rust::Object;
    // clojure::rust::ObjResult;
    clojure::rust::IObject;
    clojure::rust::Runnable;
}

init_obj! {
    Thread {
        // clojure::rust::Object::init();
        // clojure::rust::ObjResult::init();
        clojure::rust::IObject::init();
        clojure::rust::Runnable::init();
    }
}

pub trait Thread: IObject + Runnable {

}