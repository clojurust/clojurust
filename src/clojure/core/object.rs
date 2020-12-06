//! clojure::rust::object: Defines the Rust equvalent of Java Objects.
#![allow(non_snake_case)]

use std::{sync::*, mem::transmute};
pub use crate::clojure;

#[derive(Debug)]
pub struct Object {
    pub content: Arc<ObjectContent>,
}

trait IObject {
    fn getClass(&self) -> Object;
}

trait IObjectContent {
}

impl Object {
    pub fn new<T>(class: Object, ptr: &T) -> Object {
        Object {
            content: Arc::new(ObjectContent::new::<T>(class, ptr)),
        }
    }

    pub fn get<'im, T>(&self) -> &'i T {
        // Return reference of pointed object
        ObjectContent::get::<T>(self.content)
    }

    pub fn get_mut<'i, T: Copy>(&self) -> &'i mut T {
        // Return reference of pointed object
        ObjectContent::get_mut::<T>(self.content)
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(self.content)
    }

    pub unsafe fn init() {
        if INIT {return;}
        INIT = true;

        // Insures all is initialized
        Class::init();
    }
}

static mut INIT: bool = false;

impl IObjectContent for Object {
}

impl IObject for Object {
    fn getClass(&self) -> Object {
        // self.content.read().unwrap().class
        self.content.class.clone()
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

impl IObjectContent for ObjectContent 
{
}

