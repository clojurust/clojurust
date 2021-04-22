//! Protocol IEditableCollection

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait IEditableCollection: IObject {
    /// IEditableCollection -> ITransientCollection
    fn asTransient(&self) -> ObjResult<Object>;
}