
use crate::clojure;
// use clojure::rust::Object::*;
use clojure::rust::List::*;
use clojure::rust::Serializable::*;
use clojure::lang::IHashEq::*;
use clojure::lang::ISeq::*;
use clojure::lang::Sequential::*;

use crate::*;

use_obj! {
    clojure::rust::IObject;
    clojure::rust::List;
    clojure::rust::Serializable;
    clojure::lang::IHashEq;
    clojure::lang::ISeq;
    clojure::lang::Sequential;
}

init_obj! {
    ASeq {
        clojure::rust::IObject::init();
        clojure::rust::List::init();
        clojure::rust::Serializable::init();
        clojure::lang::IHashEq::init();
        clojure::lang::ISeq::init();
        clojure::lang::Sequential::init();
    }
}

pub trait ASeq: IObject + ISeq + Sequential + List + Serializable + IHashEq {

}