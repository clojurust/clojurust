use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    AFn {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::Callable::init();
    }
}

pub trait IFn: IObject + Callable {

}