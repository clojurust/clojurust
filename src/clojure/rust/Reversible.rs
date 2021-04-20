//! Protocol Reversible

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

init_obj! {
    Reversible {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}


pub trait Reversible: IObject {
    /// Reversible -> ISeq
    fn rseq(&self) -> ObjResult<Object>;
}

