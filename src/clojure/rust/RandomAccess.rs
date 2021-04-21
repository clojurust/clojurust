

use crate::*;

use_obj! {
    // clojure::rust::Object;
    clojure::rust::IObject;
    // clojure::rust::Class;
}

init_obj! {
    RandomAccess {
        // clojure::rust::Object::init();
        clojure::rust::IObject::init();
        // clojure::rust::Class::init();
    }
}

pub trait RandomAccess: IObject {}