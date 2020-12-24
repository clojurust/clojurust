//! clojure::rust::object: Defines the Rust equvalent of Java Objects.
#![allow(non_snake_case)]

use std::{sync::*, mem::transmute};
use intertrait::*;
use intertrait::cast::*;

pub use crate::clojure;
use clojure::core::class::*;
use clojure::core::iobject::*;
use clojure::core::rust_obj::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(IObject)]
pub struct Object {
    pub content: usize,
}

impl Object {
    pub fn new<T>(class: Box<dyn IObject>, ptr: Arc<T>) -> Object {
        unsafe {
            Object {
                class,
                ptr: transmute::<Arc<T>, usize>(ptr.clone()),
            }
        }
    }

    pub fn get<T>(&self) -> Arc<T> {
        unsafe {
            // Return reference of pointed object
            transmute::<usize, Arc<T>>(self.content)
        }
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(&self.content)
    }

    pub unsafe fn init() {
        if INIT {return;}
        INIT = true;

        // Insures all is initialized
        Class::init();
    

}

static mut INIT: bool = false;

trait IObject {
    fn get_class(&self) -> Object;
    fn call(&self, args: &Objects[]) -> Object;
    fn get(&self, name: &str) -> Object;
    fn is_class(&self, &str) -> bool;
    fn is_protocol(&self, &str) -> bool;
    fn to_string(&self) -> String;
    fn get_hash(&self) -> usize;
}

struct Null {}

impl IObject for Null {
    fn getClass(&self) -> Object {
        // self.content.read().unwrap().class
        RustObj::get(self.content.class.clone())
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            content: self.content.clone(),
        }
    }
}
