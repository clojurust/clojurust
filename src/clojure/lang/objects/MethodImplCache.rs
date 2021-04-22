

use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

init_obj! {
    MethodImplCache {
        clojure::rust::IObject::init();
    }
}

pub trait MethodImplCache: IObject {

}