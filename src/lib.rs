//
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
#![allow(dead_code)]
#![allow(unused_variables)]
// ![warn(unreachable_pub, missing_docs)]
#![allow(missing_docs)]
// #![allow(unused_imports)]
#![allow(bare_trait_objects)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![recursion_limit = "256"]
// For Iterator
#![feature(rustc_attrs)]
#![feature(iter_advance_by)]
#![feature(iter_intersperse)]
#![feature(iter_map_while)]
#![feature(iter_partition_in_place)]
#![feature(iter_is_partitioned)]
#![feature(try_trait)]
#![feature(try_blocks)]
#![feature(control_flow_enum)]
#![feature(try_find)]
#![feature(cmp_min_max_by)]
#![feature(iter_order_by)]
#![feature(associated_type_defaults)]

// #![feature(fn_traits)]
// #![feature(trace_macros)]
// trace_macros!(true);

/// Clojure Module
///
/// description de clojure
pub mod clojure
{
    /// Clojure language module
    ///
    /// clojure::lang modules
    pub mod lang
    {
        pub mod objects
        {
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
            pub mod IMapIterable;
            pub mod IMeta;
            pub mod IObj;
            pub mod IPersistentCollection;
            pub mod IPersistentMap;
            pub mod IPersistentSet;
            pub mod IPersistentStack;
            pub mod IPersistentVector;
            pub mod ISeq;
            pub mod ITransientCollection;
            pub mod IndexedSeq;
            pub mod MapEntry;
            pub mod MapEquivalence;
            pub mod MethodImplCache;
            pub mod PersistentHashMap;
            pub mod PersistentHashSet;
            pub mod PersistentVector;
            pub mod RSeq;
            pub mod RT;
            pub mod Sequable;
            pub mod Sequential;
            pub mod SubVector;
            pub mod Util;
        }

        pub use objects::AFn::*;
        pub use objects::AFunction::*;
        pub use objects::AMapEntry::*;
        pub use objects::APersistentMap::*;
        pub use objects::APersistentSet::*;
        pub use objects::APersistentVector::*;
        pub use objects::ASeq::*;
        pub use objects::Fn::*;
        pub use objects::IEditableCollection::*;
        pub use objects::IFn::*;
        pub use objects::IHashEq::*;
        pub use objects::IKVReduce::*;
        pub use objects::IMapEntry::*;
        pub use objects::IMapIterable::*;
        pub use objects::IMeta::*;
        pub use objects::IObj::*;
        pub use objects::IPersistentCollection::*;
        pub use objects::IPersistentMap::*;
        pub use objects::IPersistentSet::*;
        pub use objects::IPersistentStack::*;
        pub use objects::IPersistentVector::*;
        pub use objects::ISeq::*;
        pub use objects::ITransientCollection::*;
        pub use objects::IndexedSeq::*;
        pub use objects::MapEntry::*;
        pub use objects::MapEquivalence::*;
        pub use objects::MethodImplCache::*;
        pub use objects::PersistentHashMap::*;
        pub use objects::PersistentHashSet::*;
        pub use objects::PersistentVector::*;
        pub use objects::RSeq::*;
        pub use objects::Sequable::*;
        pub use objects::Sequential::*;
        pub use objects::SubVector::*;
        pub use objects::Util::*;
        pub use objects::RT::*;
    }

    /// clojure::rust host modules
    pub mod rust
    {
        pub mod objects
        {
            pub mod Associative;
            pub mod Callable;
            pub mod Class;
            pub mod Collection;
            pub mod Comparable;
            pub mod Comparator;
            pub mod Counted;
            pub mod FnMethodNative;
            pub mod Function;
            pub mod Globals;
            pub mod ILookup;
            pub mod IObject;
            pub mod Indexed;
            pub mod Iterable;
            pub mod List;
            pub mod Map;
            pub mod Member;
            pub mod Number;
            pub mod ObjResult;
            pub mod Object;
            pub mod Protocol;
            pub mod Prototype;
            pub mod RandomAccess;
            pub mod Reversible;
            pub mod Runnable;
            pub mod Serializable;
            pub mod Set;
            pub mod String;
            pub mod Thread;
            pub mod Unique;
        }
        pub mod macros
        {
            pub mod macros;
            pub mod number;
        }

        pub use objects::Associative::*;
        pub use objects::Callable::*;
        pub use objects::Class::*;
        pub use objects::Collection::*;
        pub use objects::Comparable::*;
        pub use objects::Comparator::*;
        pub use objects::Counted::*;
        pub use objects::FnMethodNative::*;
        pub use objects::Function::*;
        pub use objects::Globals::*;
        pub use objects::ILookup::*;
        pub use objects::IObject::*;
        pub use objects::Indexed::*;
        pub use objects::Iterable::*;
        pub use objects::List::*;
        pub use objects::Map::*;
        pub use objects::Member::*;
        pub use objects::Number::*;
        pub use objects::ObjResult::*;
        pub use objects::Object::*;
        pub use objects::Protocol::*;
        pub use objects::Prototype::*;
        pub use objects::RandomAccess::*;
        pub use objects::Reversible::*;
        pub use objects::Runnable::*;
        pub use objects::Serializable::*;
        pub use objects::Set::*;
        pub use objects::String::*;
        pub use objects::Thread::*;
        pub use objects::Unique::*;
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn it_works()
    {
        assert_eq!(2 + 2, 4);
    }
}
