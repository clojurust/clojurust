/// Protocol `Counted`

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
}

init_obj! {
    Counted {
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
    }
}

pub trait Counted: TObject {
    /// give the nr of elements
    fn count(&self) -> ObjResult<usize>;
}