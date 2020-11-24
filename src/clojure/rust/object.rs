//! clojure::rust::object: Defines the Rust equvalent of Java Objects.

use std::{sync::*, mem::transmute};

#[derive(Debug)]
pub struct Object {
    pub content: Arc<RwLock<ObjectContent>>,
}

impl Object {
    pub fn new<T>(class: usize, ptr: &T) -> Object {
        Object {
            content: Arc::new(RwLock::new(ObjectContent::new::<T>(class, ptr))).clone(),
        }
    }

    pub fn get<'i, T: Copy>(&self) -> &'i T {
        // Return reference of pointed object
        ObjectContent::get::<T>(&self.content.read().unwrap())
    }

    pub fn get_mut<'i, T: Copy>(&self) -> &'i mut T {
        // Return reference of pointed object
        ObjectContent::get_mut::<T>(&self.content.write().unwrap())
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(&self.content)
    }


}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            content: self.content.clone(),
        }
    }
}


/// Object struture stores various data.
#[derive(Debug)]
pub struct ObjectContent {
    /// Keyword index of the `Class` of `Object`.
    pub class: usize,
    /// Reference to the `Object` as a raw pointer.
    pub ptr: usize,
}

impl ObjectContent {
    pub fn new<T>(class: usize, ptr: &T) -> ObjectContent {
        unsafe {
            ObjectContent {
                class,
                ptr: transmute::<&T, usize>(ptr),
            }
        }
    }

    pub fn get<'i, T: Copy>(&self) -> &'i T {
        unsafe {
            // Return reference of pointed object
            &*transmute::<usize, &T>(self.ptr)
        }
    }

    pub fn get_mut<'i, T: Copy>(&self) -> &'i mut T {
        unsafe {
            // Return reference of pointed object
            transmute::<usize, &mut T>(self.ptr)
        }
    }
}
