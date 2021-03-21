//! # Defines Rust dynamic `Object`s.
//!
//! Define dynamic `Object`s as Option<Arc<TObject>>

use std::clone::Clone;
use std::{fmt::*, hash::*, sync::*};

use intertrait::cast::CastArc;

// use std::fmt::*;
use crate::use_obj;

use_obj! {
    clojure::rust::class;
    clojure::rust::nil;
}

castable_to!(Object => [sync] TObject, TObject);

init_obj! {
    Object {
        clojure::rust::class::init();
        clojure::rust::nil::init();
    }
}

pub trait Inner: TObject + Debug + Eq + Hash + CastFromSync {}

pub struct Object {
    pub inner: Arc<TObject>,
}

castable_to!(Object => [sync] TObject);

impl<'a> Object
where
    Object: TObject + 'a,
{
    pub fn new(obj: Arc<TObject>) -> Object {
        Object { inner: obj }
    }

    pub fn null() -> Object {
        Nil::new()
    }

    pub fn is_null(&self) -> bool {
        ::isa::<TNil>(self)
    }

    pub fn isa<T>(&'static self) -> bool
    where
        T: 'static,
    {
        let b = self.clone();
        let c = b.inner;
        let a = CastArc::cast::<T>(c);
        match a {
            Ok(o) => true,
            Err(oo) => false,
        }
    }

    pub fn cast<T>(&'a self) -> Option<&'a T>
    where
        T: 'static,
    {
        let b = self.clone();
        let c = b.inner;
        let a = CastArc::cast::<T>(c);
        match a {
            Ok(o) => {
                let t = Some(o.as_ref());
                t
            }
            Err(oo) => None,
        }
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(&self.inner)
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
        let a = self.clone();
        let b = a.get_class();
        b.get_class().get(name).clone()
    }

    pub fn get_by_name(&self, name: &str) -> Object {
        let a = self.clone();
        let b = a.get_class();
        b.get_class().get(name).clone()
    }

    pub fn set_by_id(&self, name: usize, value: Object) -> Object {
        let a = self.clone();
        let b = a.get_class();
        b.get_class().set(name, value).clone()
    }

    pub fn set_by_name(&self, name: &str, value: Object) -> Object {
        let a = self.clone();
        let b = a.get_class();
        b.get_class().set(name, value).clone()
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
        self.clone().inner.get_class()
    }

    fn to_string(&self) -> &str {
        let a = self.clone();
        a.get_class().to_string()
    }

    fn get_hash(&self) -> usize {
        let a = self.clone();
        a.get_class().get_hash()
    }

    fn equals(&self, other: &Object) -> bool {
        let a = self.clone();
        a.get_class().equals(other)
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
        Object {
            inner: inner.clone(),
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
