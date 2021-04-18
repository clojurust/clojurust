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
        pub mod AFn;
        pub mod AFunction;
        pub mod AMapEntry;
        pub mod APersistentMap;
        pub mod APersistentSet;
        pub mod APersistentVector;
        pub mod ASeq;
        pub mod Fn;
        pub mod IEditableCollection;
        pub mod IFn;
        pub mod IHashEq;
        pub mod IKVReduce;
        pub mod IMapEntry;
        pub mod IMeta;
        pub mod IndexedSeq;
        pub mod IObj;
        pub mod IPersistentCollection;
        pub mod IPersistentMap;
        pub mod IPersistentStack;
        pub mod IPersistentVector;
        pub mod ISeq;
        pub mod ITransientCollection;
        pub mod MapEquivalence;
        pub mod PersistentMap;
        pub mod PersistentSet;
        pub mod PersistentVector;
        pub mod RT;
        pub mod Sequable;
        pub mod Sequential;
    }

    /// Rust host Module
    pub mod rust {
        pub mod Associative;
        pub mod Callable;
        pub mod Class;
        pub mod Comparable;
        pub mod Comparator;
        pub mod Counted;
        pub mod FnMethodNative;
        pub mod Function;
        pub mod Globals;
        pub mod Indexed;
        pub mod Iterable;
        pub mod List;
        pub mod Map;
        pub mod Member;
        pub mod Number;
        pub mod Object;
        pub mod ObjError;
        pub mod Protocol;
        pub mod Prototype;
        pub mod RandomAccess;
        pub mod Reversible;
        pub mod Runnable;
        pub mod Serializable;
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
