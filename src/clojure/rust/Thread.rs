
use crate::*;

use_obj! {
    // clojure::rust::Object;
    // clojure::rust::ObjResult;
    clojure::rust::IObject;
}

init_obj! {
    Thread {
        // clojure::rust::Object::init();
        // clojure::rust::ObjResult::init();
        clojure::rust::IObject::init();
    }
}

pub trait Thread: IObject {

}