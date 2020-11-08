//! Defines the Java Object

use rc::Rc;
use str::Ptr;

/// Object struture stores various data.
#[allow(dead_code)]
pub struct Object {
    value_type: usize,
    value: Rc<Ptr>,
}

#[allow(dead_code)]
impl Object {
    pub fn new(value_type: usize, obj: Ptr) -> Object {
        Object {
            value_type
            value: Rc::new(obj),
        }
    }
}
