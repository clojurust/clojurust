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
    fn get<'i, T: Copy>(&self) -> &'i T;
    fn get_mut<'i, T: Copy>(&self) -> &'i mut T;
}

trait IContentObject {
    fn getClass(&self) -> Option<&Object>;
    fn hashCode(&self) -> usize;
    fn equals(&self, other: &Object) -> bool;
    fn toString(&self) -> String;
}

impl Object {
    pub fn new<T>(class: usize, ptr: &T) -> Object {
        Object {
            content: Arc::new(RwLock::new(ObjectContent::new::<T>(class, ptr))).clone(),
        }
    }

    fn count(&self) -> usize {
        Arc::strong_count(&self.content)
    }
}

impl IContentObject for Object {
    fn getClass(&self) -> Option<&Object> {
        let class = (&self.content.read().unwrap()).class;
        super::rust_obj::init();
        super::rust_obj::RUST_OBJ.objects.get(&9)
    }
    
    fn hashCode(&self) -> usize {0}
    fn equals(&self,other: &Object) -> bool{true}
    fn toString(&self) -> String {"lkjlkjlkjl".to_string()}
}

impl IObject for Object {
    fn get<'i, T: Copy>(&self) -> &'i T {
        // Return reference of pointed object
        ObjectContent::get::<T>(&self.content.read().unwrap())
    }

    fn get_mut<'i, T: Copy>(&self) -> &'i mut T {
        // Return reference of pointed object
        ObjectContent::get_mut::<T>(&self.content.write().unwrap())
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
}

impl IObject for ObjectContent {
    fn get<'i, T: Copy>(&self) -> &'i T {
        unsafe {
            // Return reference of pointed object
            &*transmute::<usize, &T>(self.ptr)
        }
    }

    fn get_mut<'i, T: Copy>(&self) -> &'i mut T {
        unsafe {
            // Return reference of pointed object
            transmute::<usize, &mut T>(self.ptr)
        }
    }
}

impl IContentObject for ObjectContent {
    fn getClass(&self) -> Option<&Object> {None}
    fn hashCode(&self) -> usize {0}
    fn equals(&self,other: &Object) -> bool{true}
    fn toString(&self) -> String {"".to_string()}
}