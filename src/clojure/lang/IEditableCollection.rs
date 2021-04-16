//! Protocol IEditableCollection

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::ITransientCollection::*;

pub trait IEditableCollection: TObject {
    fn asTransient(&self) -> ObjResult<&'_ ITransientCollection>;
}