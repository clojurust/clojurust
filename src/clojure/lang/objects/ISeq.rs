//! Protocol ISeq

use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait ISeq: IObject+Sequable+IPersistentCollection
{
    /// ISeq -> Object -> ISeq
    fn cons(
        &self,
        o: &Object,
    ) -> ObjResult<Object>;
}
