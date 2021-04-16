//Protocol IObj

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;
use clojure::lang::IMeta::*;

pub trait IObj: TObject + IMeta {
    fn withMeta(&self, meta: &Object) -> ObjResult<&'_ IObj>;
}