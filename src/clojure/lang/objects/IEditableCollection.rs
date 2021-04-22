//! Protocol IEditableCollection

use crate::*;
use clojure::rust::*;
use clojure::lang::*;

init_obj! {
    AFn {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Callable::init();
        clojure::rust::Runnable::init();
    }
}

pub trait IEditableCollection: IObject {
    /// IEditableCollection -> ITransientCollection
    fn asTransient(&self) -> ObjResult<Object>;
}