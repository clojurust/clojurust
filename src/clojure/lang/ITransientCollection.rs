//! Prototocol ITransientCollection

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;

pub trait ITransientCollection: TObject {
    fn conj(&self, val: &Object) -> ObjResult<&'_ ITransientCollection>;
    fn persistant(&self) -> ObjResult<&'_ ITransientCollection>;
}