//! clojure::rust::object: Defines the Rust equvalent of Java Objects.

use std::{sync::Arc, mem::transmute};

/// Object struture stores various data.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Object<'i> {
    /// Keyword index of the `Class` of `Object`.
    class: usize,
    /// Value of the `Object` as a raw pointer.
    ptr: Arc<&'i ()>,
}

#[allow(dead_code)]
impl Object<'_> {
    pub fn new<T>(class: usize, obj: &T) -> Object {
        unsafe {
            Object {
                class,
                ptr: Arc::new(transmute::<&T, &()>(obj)),
            }
        }
    }

}

impl<'i> Object<'i> {
    pub fn get<T>(obj: &Object) -> &'i T {
    unsafe {
        transmute::<&(), &T>(*obj.ptr)
    }
}
