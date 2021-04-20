//Protocol IObj

use crate::*;

use_obj! {
    clojure::lang::IMeta;
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

init_obj! {
    IObj {
        clojure::lang::IMeta::init();
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait IObj: IObject + IMeta {
    fn withMeta(&self, meta: &Object) -> ObjResult<Object>;
}