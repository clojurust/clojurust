use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

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