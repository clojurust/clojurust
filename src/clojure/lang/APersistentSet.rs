//! # HashSet of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashSet<Object> library

use crate::*;

use_obj! {
    // clojure::rust::Object; 
    clojure::rust::IObject;
    clojure::lang::IPersistentSet;
    clojure::lang::AFn;
    clojure::rust::Collection;
    clojure::lang::Set;
    clojure::rust::Serializable;
    clojure::lang::IHashEq;

}

init_obj! {
    APersistentSet {
    }
}

pub trait APersistentSet: IObject + AFn + IPersistentSet 
                + Collection + Set + Serializable + IHashEq {
    
}
