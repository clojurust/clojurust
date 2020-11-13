//! clojure::rust::object: Defines the Rust equvalent of Java Objects.

use rc::Rc;

/// Object struture stores various data.
#[allow(dead_code)]
pub struct Object {
    /// keyword index of the `Class` of `Object`.
    class: usize,
    /// value of the `Object` as a raw pointer.
    ptr: Rc<usize>,
}

#[allow(dead_code)]
impl Object {
    pub fn new(class: usize, ptr: usize) -> Object {
        Object {
            class,
            ptr: Rc::new(ptr),
        }
    }
}
