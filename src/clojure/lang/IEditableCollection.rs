//! Protocol IEditableCollection

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::rust::Callable;
    clojure::rust::Runnable;
}

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