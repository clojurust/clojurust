//Protocol IObj

use crate::*;

use_obj! {
    clojure::lang::IMeta;
    clojure::rust::Object;
    clojure::rust::ObjError;
}

init_obj! {
    Runnable {
        clojure::lang::IMeta::init();
        clojure::rust::Object::init();
        clojure::rust::ObjError::init();
    }
}

pub trait IObj: TObject + IMeta {
    fn withMeta(&self, meta: &Object) -> ObjResult<Object>;
}