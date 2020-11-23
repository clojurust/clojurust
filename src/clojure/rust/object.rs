//! clojure::rust::object: Defines the Rust equvalent of Java Objects.

use std::{sync::Arc, mem::transmute};

/// Object struture stores various data.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Object {
    /// Keyword index of the `Class` of `Object`.
    pub class: usize,
    /// Value of the `Object` as a raw pointer.
    pub ptr: usize,
}

#[allow(dead_code)]
impl Object {
    pub fn new<T>(class: usize, obj: T) -> Arc<Object> {
        unsafe {
            Arc::new(Object {
                class,
                ptr: transmute::<&T, usize>(&Arc::new(obj)),
            })
        }
    }
}

impl Object {
    pub fn get<T: Copy>(obj: &Object) -> Arc<T> {
        unsafe {
            Arc::<T>::from_raw(transmute::<usize, *const T>(obj.ptr))
        }
    }
}
