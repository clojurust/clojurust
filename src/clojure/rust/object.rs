//! clojure::rust::object: Defines the Rust equvalent of Java Objects.
#![allow(non_snake_case)]

use std::{sync::*, mem::transmute};
use std::option::*;
pub use crate::clojure;

#[derive(Debug)]
pub struct Object {
    pub content: Arc<RwLock<ObjectContent>>,
}

trait IObject {
    fn getClass(&self) -> Option<&Object>;
}

trait IObjectContent {
    fn hashCode(&self) -> usize;
    fn equals(&self, other: &Object) -> bool;
    fn toString(&self) -> String;
}

impl Object {
    pub fn new<T>(class: usize, ptr: &T) -> Object {
        Object {
            content: Arc::new(RwLock::new(ObjectContent::new::<T>(class, ptr))),
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

impl IObjectContent for Object {
    fn hashCode(&self) -> usize {
        self.content.read().unwrap().hashCode()
    }

    fn equals(&self,other: &Object) -> bool{
        self.content.read().unwrap().equals(other)
    }

    fn toString(&self) -> String {
        self.content.read().unwrap().toString()
    }
}

impl IObject for Object {
    fn getClass(&self) -> Option<&Object> {
        // self.content.read().unwrap().class
        None
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
    // unimplemented for now
    fn hashCode(&self) -> usize {0}

    fn equals(&self,other: &Object) -> bool {true}

    fn toString(&self) -> String {"".to_string()}
}