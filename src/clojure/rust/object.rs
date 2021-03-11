//! # Defines Rust dynamic Objects.
//!
//! Define immutable dynamic objects
//!

// use lazy_static::{__Deref, lazy_static};
use std::{
    borrow::{Borrow, BorrowMut},
    clone::Clone,
};
// use std::{any::*, convert::*, result::*};
use std::{fmt::*, hash::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;

pub trait Inner: TObject + Debug + Eq + Hash + CastFromSync {}

/// Basic definition of object inner link to real structure
// pub type Inner = IObject;

/// Basic definition of a dynamic object
pub struct Object {
    pub inner: Option<Arc<TObject>>,
}

castable_to!(Object => [sync] TObject);

impl<'a> Object {
    pub fn new<T: TObject>(obj: T) -> Object {
        Object {
            inner: Some(Arc::new(obj)),
        }
    }

    pub fn null() -> Object {
        Object { inner: None }
    }

    pub fn is_null(&self) -> bool {
        match self.inner {
            None => true,
            Some(_) => false,
        }
    }

    pub fn inn<T>(&'a mut self) -> &'a T
    where
        T: TObject,
    {
        let a = self.inner.borrow();
        a.cast::<T>().expect("Unexpected error")
    }

    pub fn inn_mut<T>(&'a mut self) -> &'a mut T
    where
        T: TObject,
    {
        let a = self.inner.borrow_mut();
        a.cast::<T>().expect("Unexpected error")
    }
}

/// `Object` `Protocol` for all defined `Object`s
///
///
pub trait TObject: CastFromSync {
    /// Return `Class` of `Object`
    fn get_class<'a>(&self) -> &'a SClass;

    fn to_string(&self) -> &str;

    fn get_hash(&self) -> usize;

    fn equals(&self, other: &Object) -> bool;
}

const NILSTRING: &str = "nil";

impl Class for Object {
    fn call(&self, name: usize, args: &[Object]) -> Object {
        match self.clone().inner {
            None => panic!("Call on nil"),
            Some(o) => {
                let a = o.clone();
                o.get_class().call(name, args).clone()
            }
        }
    }

    fn get(&self, name: usize) -> Object {
        match self.clone().inner {
            None => panic!("Getter on nil"),
            Some(o) => {
                let a = o.clone();
                let b = a.get_class();
                b.get_class().get(name).clone()
            }
        }
    }
}

/// SImplementation of protocol IObject for Object.
///
/// Functions are applied to the `content` of `Object`
// #[cast_to([sync] IObject, Debug)];
impl TObject for Object {
    fn get_class<'a>(&self) -> &'a SClass {
        if let Some(o) = self.clone().inner {
            o.clone().get_class()
        } else {
            panic!("Get class from nil")
        }
    }

    fn to_string(&self) -> &str {
        match self.clone().inner {
            None => NILSTRING,
            Some(o) => {
                let a = o.clone();
                a.get_class().to_string()
            }
        }
    }

    fn get_hash(&self) -> usize {
        match self.clone().inner {
            None => 0,
            Some(o) => {
                let a = o.clone();
                a.get_class().get_hash()
            }
        }
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self.clone();
        f.write_str(a.to_string())
    }
}

impl Clone for Object {
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }

    fn clone(&self) -> Self {
        let Object { inner } = self;
        match inner {
            None => Object { inner: None },
            Some(o) => Object {
                inner: Some(o.clone()),
            },
        }
    }
}

impl Eq for Object {}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.get_hash())
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state);
        }
    }
}

impl Hash for TObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.get_hash())
    }
}

pub unsafe fn init() {
    // only execute one time
    if INIT {
        return;
    }

    INIT = true;

    println!("Object::init");

    // Insures all is initialized
    clojure::rust::class::init();
}

static mut INIT: bool = false;
