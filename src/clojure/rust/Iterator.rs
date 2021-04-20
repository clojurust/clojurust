/// Iterator

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::rust::Iterator;
}

init_obj! {
    Function {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Iterator::init();
    }
}

pub trait Iterator: IObject {
    /// Iterator -> bool
    fn hasNext(&self) -> ObjResult<Object>;
    
    /// Iterator -> Object
    fn next(&self) -> ObjResult<Object>;
    
    /// Iterator
    fn remove(&self) -> ObjResult<Object>;
}