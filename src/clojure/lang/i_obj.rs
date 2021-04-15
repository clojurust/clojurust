//Protocol IObj

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;
use clojure::lang::i_persistent_map::*;
use clojure::lang::i_meta::*;

pub trait IObj: TObject + IMeta {
    fn withMeta(&self, meta: IPersistentMap) -> ObjResult<&'_ IObj>;
}