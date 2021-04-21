
use crate::*;

use_obj! {
    clojure::rust::IObject;
}

init_obj! {
    Runnable {
        clojure::rust::IObject::init();
    }
}

pub trait Serializable: IObject {}