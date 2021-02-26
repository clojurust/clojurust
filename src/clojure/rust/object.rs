//! # Defines Rust dynamic Objects.
//!
//! Define immutable dynamic objects
//!

#![allow(non_snake_case)]

use lazy_static::{__Deref, lazy_static};
use std::clone::Clone;
use std::{any::*, fmt::*, hash::*, result::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::class::*;
// use super::rust_obj::*;

pub trait Inner: Object + Debug + Eq + Hash + CastFromSync {}

/// Basic definition of object inner link to real structure
// pub type Inner = IObject;

/// Basic definition of a dynamic object
// pub type Object = Option<Arc<Inner>>;
pub struct SObject {
    pub inner: Option<Arc<Object>>,
}

castable_to!(SObject => [sync] Object);

impl SObject {
    pub fn new<T: Object>(obj: T) -> SObject {
        SObject {
            inner: Some(Arc::new(obj)),
        }
    }

    pub fn null() -> SObject {
        SObject { inner: None }
    }

    pub fn is_null(&self) -> bool {
        match self.inner {
            None => true,
            Some(_) => false,
        }
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }

        INIT = true;

        println!("Class::init");

        // Insures all is initialized
        Class::init();
    }
}

/// `IObject` `Protocol` for all defined `Object`s
///
///
pub trait Object: CastFromSync {
    /// Return `Class` of `Object`
    fn get_class<'a>(&'a self) -> &'a Class;

    /// Call named `method` with `Object`s arguments
    fn call(&self, name: &str, args: &[SObject]) -> SObject;

    /// Call getter for a named `member`
    fn get(&self, name: &str) -> SObject;

    /// Return string representation of
    fn to_string(&self) -> String;

    fn get_hash(&self) -> usize;
}

const NILSTRING: &str = "nil";

/// Implementation of protocol IObject for Object.
///
/// Functions are applied to the `content` of `Object`
// #[cast_to([sync] IObject, Debug)];
impl Object for SObject {
    fn get_class<'a>(&'a self) -> &'a Class {
        if let Some(o) = self.clone().inner {
            o.get_class()
        } else {
            panic!("Get class from nil")
        }
    }

    fn call(&self, name: &str, args: &[SObject]) -> SObject {
        match self.clone().inner {
            None => panic!("Call on nil"),
            Some(o) => {
                let a = o.clone();
                o.get_class().call(name, args)
            }
        }
    }

    fn get(&self, name: &str) -> SObject {
        match self.clone().inner {
            None => panic!("Getter on nil"),
            Some(o) => {
                let a = o.clone();
                let b = a.get_class();
                b.get(name)
            }
        }
    }

    fn to_string(&self) -> String {
        match self.clone().inner {
            None => String::from(NILSTRING),
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
}

impl<T> Object for T
where
    T: Inner,
{
    fn get_class<'a>(&'a self) -> &'a Class {
        todo!()
    }

    fn call(&self, name: &str, args: &[SObject]) -> SObject {
        todo!()
    }

    fn get(&self, name: &str) -> SObject {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }
}

impl std::fmt::Debug for SObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self;
        f.write_str(&self.to_string())
    }
}

impl Clone for SObject {
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }

    fn clone(&self) -> Self {
        let SObject { inner } = self;
        match inner {
            None => SObject { inner: None },
            Some(o) => SObject {
                inner: Some(o.clone()),
            },
        }
    }
}

impl Eq for SObject {}

impl PartialEq for SObject {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Hash for SObject {
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

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.get_hash())
    }
}

static mut INIT: bool = false;
