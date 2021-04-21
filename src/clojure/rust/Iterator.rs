

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
}

init_obj! {
    List {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
    }
}

pub trait Iterator: IObject {
    fn hasNext(&self) -> ObjResult<bool>;
    fn next(&self) -> ObjResult<Object>;
    
    // Unimplemented
    // fn remove()
}