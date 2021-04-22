
use crate::*;
use clojure::rust::*;
use clojure::lang::*;

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