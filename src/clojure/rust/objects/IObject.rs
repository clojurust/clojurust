use clojure::rust::*;
// use clojure::lang::*;
/// `IObject` `Protocol` for all defined `Object`s
///
use intertrait::*;

/// IObject
use crate::*;
pub trait IObject: CastFromSync
{
    /// Return `Class` of `Object`
    fn getClass<'a>(&self) -> &'a SClass;

    fn hashCode(&self) -> usize;

    fn toString(&self) -> String;

    fn equals(
        &self,
        other: &Object,
    ) -> bool;
}
