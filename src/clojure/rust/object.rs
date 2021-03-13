//! # Defines Rust dynamic `Object`s.
//!
//! Define dynamic `Object`s as Option<Arc<TObject>>

// use lazy_static::{__Deref, lazy_static};
use std::{borrow::BorrowMut, clone::Clone};
// use std::{any::*, convert::*, result::*};
use std::{fmt::*, hash::*, sync::*};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::nil::*;

pub trait Inner: TObject + Debug + Eq + Hash + CastFromSync {}

pub struct Object {
    pub inner: Arc<TObject>,
}

castable_to!(Object => [sync] TObject);

impl<'a> Object {
    pub fn new(obj: Arc<TObject>) -> Object {
        Object { inner: obj }
    }

    pub fn null() -> Object {
        Nil::new()
    }

    pub fn is_null(&self) -> bool {
        self.isa::<Nil>()
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    pub fn isa<T>(&self) -> bool
    where
        T: TObject + 'static,
    {
        self.inner.as_ref().impls::<T>()
    }

    pub fn inn<T>(&self) -> &T
    where
        T: TObject + ?Sized + 'static,
    {
        let a = self.clone().inner;
        match a.cast::<T>() {
            Ok(b) => b.clone().as_ref(),
            _ => unreachable!(),
        }
    }

    pub fn inn_mut<T>(&self) -> &mut T
    where
        T: TObject + ?Sized + 'static,
    {
        self.inn::<T>().borrow_mut()
    }

    pub fn call_by_id(&self, name: usize, args: &[Object]) -> Object {
        let a = self.clone().inner;
        {
            a.get_class().call(name, args).clone()
        }
    }

    pub fn call_by_name(&self, name: &str, args: &[Object]) -> Object {
        let a = self.clone().inner;
        {
            a.get_class().call(name, args).clone()
        }
    }

    pub fn get_by_id(&self, name: usize) -> Object {
        match self.clone().inner {
            None => panic!("Getter on nil"),
            Some(o) => {
                let a = o.clone();
                let b = a.get_class();
                b.get_class().get(name).clone()
            }
        }
    }

    pub fn get_by_name(&self, name: &str) -> Object {
        match self.clone().inner {
            None => panic!("Getter on nil"),
            Some(o) => {
                let a = o.clone();
                let b = a.get_class();
                b.get_class().get(name).clone()
            }
        }
    }

    pub fn set_by_id(&self, name: usize, value: Object) -> Object {
        match self.clone().inner {
            None => panic!("Getter on nil"),
            Some(o) => {
                let a = o.clone();
                let b = a.get_class();
                b.get_class().set(name, value).clone()
            }
        }
    }

    pub fn set_by_name(&self, name: &str, value: Object) -> Object {
        match self.clone().inner {
            None => panic!("Getter on nil"),
            Some(o) => {
                let a = o.clone();
                let b = a.get_class();
                b.get_class().set(name, value).clone()
            }
        }
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
