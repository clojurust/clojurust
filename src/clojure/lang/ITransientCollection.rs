//! Prototocol ITransientCollection

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjResult::*;

pub trait ITransientCollection: IObject {
    fn conj(&self, val: &Object) -> ObjResult<&'_ ITransientCollection>;
    fn persistant(&self) -> ObjResult<&'_ ITransientCollection>;
}