

use crate::*;

use_obj! {
    clojure::rust::IObject;
}

init_obj! {
    MethodImplCache {
        clojure::rust::IObject::init();
    }
}

pub trait MethodImplCache: IObject {

}