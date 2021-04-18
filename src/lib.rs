#![crate_name = "clojurust"]

//! # `clojurust` crate: Proof of concept for a Clojure library in Rust.
//!
//! This library implements root functions for a Rust implemented `host` for
//! manage a `Clojure` implemented in `Rust`.
//! It implements also the core Java equivalent of the Java version of
//! Clojure in Rust.
//!
//! ## `crate::clojure::rust` module
//!
//! This module contains the `Rust` host.
//!
//!
//!
#![allow(dead_code)]
#![allow(unused_variables)]
// ![warn(unreachable_pub, missing_docs)]
#![allow(missing_docs)]
// #![allow(unused_imports)]
#![allow(bare_trait_objects)]
#![allow(non_snake_case)]
#![recursion_limit = "256"]
// #![feature(fn_traits)]
// #![feature(trace_macros)]
// trace_macros!(true);

/// Clojure Module
///
/// description de clojure
pub mod clojure {
    /// Clojure language module
    ///
    /// description de lang
    pub mod lang {
        pub mod APersistentVector;
        pub mod APersistentSet;
        pub mod APersistentMap;
        pub mod AMapEntry;
        pub mod ASeq;
        pub mod PersistentVector;
        pub mod PersistentSet;
        pub mod PersistentMap;
        pub mod MapEquivalence;
        pub mod IFn;
        pub mod IObj;
        pub mod IMeta;
        pub mod IMapEntry;
        pub mod IKVReduce;
        pub mod IPersistentVector;
        pub mod IPersistentStack;
        pub mod IPersistentMap;
        pub mod IPersistentCollection;
        pub mod ITransientCollection;
        pub mod IEditableCollection;
        pub mod IndexedSeq;
        pub mod ISeq;
        pub mod IHashEq;
        pub mod Sequable;
        pub mod Sequential;
    }

    /// Rust host Module
    pub mod rust {
        pub mod Class;
        pub mod Counted;
        pub mod Comparable;
        pub mod Iterable;
        pub mod Associative;
        pub mod Serializable;
        pub mod Indexed;
        pub mod Reversible;
        pub mod FnMethodNative;
        pub mod Function;
        pub mod Globals;
        pub mod Member;
        pub mod Number;
        pub mod Map;
        pub mod List;
        pub mod RandomAccess;
        pub mod ObjError;
        pub mod Object;
        pub mod Protocol;
        pub mod Prototype;
        pub mod Stri;
        pub mod Unique;
        pub mod macros {
            pub mod macros;
            pub mod number;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
