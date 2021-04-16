
use crate::clojure;
// use clojure::rust::Object::*;
use clojure::rust::List::*;
use clojure::rust::Serializable::*;
use clojure::lang::IHashEq::*;
use clojure::lang::ISeq::*;
use clojure::lang::Sequential::*;

pub trait ASeq: ISeq + Sequential + List + Serializable + IHashEq {

}