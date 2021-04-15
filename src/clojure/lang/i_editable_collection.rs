//! Protocol IEditableCollection

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::i_transient_c0llection::*;

pub trait IEditableCollection: TObject {
    fn asTransient(&self) -> ObjResult<ITransientCollection>;
}