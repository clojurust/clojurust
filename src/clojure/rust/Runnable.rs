use crate::*;

use_obj! {
    // clojure::rust::Object;
    // clojure::rust::ObjResult;
    clojure::rust::IObject;
    clojure::rust::Callable;
    clojure::rust::Thread;
}

init_obj! {
    Runnable {
        // clojure::rust::Object::init();
        // clojure::rust::ObjResult::init();
        clojure::rust::IObject::init();
        clojure::rust::Callable::init();
        clojure::rust::Thread::init();
    }
}

pub trait Runnable: IObject + Callable + Thread {
    fn run(&self);
}