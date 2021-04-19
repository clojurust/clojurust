/// Protocol `Counted`

use crate::clojure;
use clojure::rust::Object::*;
use clojure::rust::ObjError::*;

use crate::*;

init_obj! {
    Counted {
    }
}

pub trait Counted: TObject {
    /// give the nr of elements
    fn count(&self) -> ObjResult<usize>;
}