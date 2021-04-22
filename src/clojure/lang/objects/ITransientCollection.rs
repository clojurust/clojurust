//! Prototocol ITransientCollection

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait ITransientCollection: IObject {
    /// ITransientCollection -> ITransientCollection
    fn conj(&self, val: &Object) -> ObjResult<Object>;

    /// ITransientCollection -> IPersistentCollection
    fn persistant(&self) -> ObjResult<Object>;
}