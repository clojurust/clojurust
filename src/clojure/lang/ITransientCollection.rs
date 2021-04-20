//! Prototocol ITransientCollection

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

init_obj! {
    ITransientCollection {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait ITransientCollection: IObject {
    /// ITransientCollection -> ITransientCollection
    fn conj(&self, val: &Object) -> ObjResult<Object>;

    /// ITransientCollection -> IPersistentCollection
    fn persistant(&self) -> ObjResult<Object>;
}