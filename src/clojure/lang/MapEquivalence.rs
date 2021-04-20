
use crate::*;

use_obj! {
    clojure::rust::IObject;
}

init_obj! {
    Sequential {
        clojure::rust::IObject::init();
    }
}

pub trait MapEquivalence: IObject {
    
}