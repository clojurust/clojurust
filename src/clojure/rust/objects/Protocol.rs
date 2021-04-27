//! # Protocol is the eqivalent of `traits`.
//!
//! `Protocol` defines the templates for the functions of the trait
//! for the library usage.

// use intertrait::cast::*;

use clojure::lang::*;
use clojure::rust::*;
use intertrait::*;

use crate::*;
castable_to!(SProtocol => [sync] IObject, Protocol);

pub struct SProtocol {
    inner: SPersistentHashSet,
}

impl IObject for SProtocol {
    #[allow(non_snake_case)]
    fn getClass<'a>(&self) -> &'a SClass { todo!() }

    #[allow(non_snake_case)]
    fn hashCode(&self) -> usize { todo!() }

    fn equals(
        &self,
        other: &Object,
    ) -> bool {
        todo!()
    }

    #[allow(non_snake_case)]
    fn toString(&self) -> String { todo!() }
}

pub trait Protocol: IObject {}

impl Protocol {}

impl Protocol for SProtocol {}

impl SProtocol {}
