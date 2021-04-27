//! Protocol IEditableCollection

// use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait IEditableCollection: IObject {
    /// IEditableCollection -> ITransientCollection
    fn asTransient(&self) -> ObjResult<Object>;
}
