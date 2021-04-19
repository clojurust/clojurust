//! Protocol Reversible

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
    clojure::lang::ISeq;
}

init_obj! {
    Reversible {
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
        clojure::lang::ISeq::init();
    }
}


pub trait Reversible: TObject {
    fn rseq(&self) -> ObjResult<&'_ ISeq>;
}

