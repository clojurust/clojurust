
use crate::*;
use clojure::rust::*;
use clojure::lang::*;

pub trait ASeq: IObject + ISeq + Sequential + List + Serializable + IHashEq {

}