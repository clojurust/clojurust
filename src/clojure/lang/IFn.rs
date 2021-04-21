use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::Callable;
}

init_obj! {
    AFn {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::Callable::init();
    }
}

pub trait IFn: IObject + Callable {

}