//Protocol IObj

use crate::*;

use_obj! {
    clojure::lang::IMeta;
    clojure::rust::Object;
    clojure::rust::ObjResult;
}

init_obj! {
    Runnable {
        clojure::lang::IMeta::init();
        clojure::rust::Object::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IObj: IObject + IMeta {
    fn withMeta(&self, meta: &Object) -> ObjResult<Object>;
}