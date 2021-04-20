//! Protocol IEditableCollection

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjResult::*;
use clojure::lang::ITransientCollection::*;

pub trait IEditableCollection: IObject {
    fn asTransient(&self) -> ObjResult<&'_ ITransientCollection>;
}