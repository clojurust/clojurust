/// Protocol `Counted`

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

init_obj! {
    Counted {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait Counted: IObject {
    /// give the nr of elements
    fn count(&self) -> ObjResult<usize>;
}