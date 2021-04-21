
use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::ObjResult;
    clojure::rust::Iterable;
}

init_obj! {
    Collection {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::ObjResult::init();
        clojure::rust::Iterable::init();
    }
}

pub trait Collection: IObject + Iterable {
    fn size(&self) -> usize;
    fn isEmpty(&self) -> bool;
    fn contains(&self) -> ObjResult<bool>;
    fn toArray(&self) -> ObjResult<Vec<Object>>;

    /// Collection -> Collection
    // Unimplemented
    // fn add(&self, e: &Object) -> ObjResult<Object>;
    
    // Unimplemented
    // fn remove(&self, o: Object) -> ObjResult<bool>;

    // (Collection -> Collection) -> bool
    fn containsAll(&self, c: &Object) -> ObjResult<bool>;
    
    // Unimplemented
    // fn addAll(&self, c: Collection) -> ObjResult<bool>;

    // Unimplemented
    // fn removeAll(&self, c: Collection) -> ObjResult<bool>;

    // Unimplemented
    // fn retainAll(&self, c: Collection) -> ObjResult<bool>;
    
    // Unimplemented
    // fn clear(&self) -> ObjResult<bool>;
}
