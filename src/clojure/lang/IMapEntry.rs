//! Protocol IMapEntry

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::rust::Map;
}

init_obj! {
    IMeta {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Map::init();
    }
}

pub trait IMapEntry: IObject + MapEntry {
    fn key(&self) -> ObjResult<Object>;
    fn val(&self) -> ObjResult<Object>;    
}