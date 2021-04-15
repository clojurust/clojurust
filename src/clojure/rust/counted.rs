/// Protocol `Counted`

use crate::clojure;
use clojure::rust::object::*;
use clojure::rust::obj_error::*;

use crate::init_obj;
use crate::init_init_obj;

init_obj! {
    Counted {
    }
}

pub trait Counted: TObject {
    /// give the nr of elements
    fn count(&self) -> ObjResult<usize>;
}