//! Protocol Associative

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait Associative {
    /// Associative -> Object -> Object -> Associative
    fn assoc(&self, key: &Object, value: &Object) -> ObjResult<Object>;
    
    /// Associative -> Object -> bool
    fn containsKey(&self, key: &Object) -> ObjResult<bool>;

    /// Associative -> Object -> IMapEntry
    fn entryAt(&self, key: &Object) -> ObjResult<Object>;
}
