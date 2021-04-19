use crate::*;

use_obj! {
    // clojure::rust::Object;
    // clojure::rust::Class;
    // clojure::rust::ObjError;
    clojure::rust::Callable;
    clojure::rust::Runnable;
}

init_obj! {
    AFn {
        Callable::init();
        Runnable::init();
    }
}

pub trait IFn: Callable + Runnable {

}