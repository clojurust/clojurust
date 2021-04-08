//! # Defines Rust dynamic `Object`s.
//!
//! Define dynamic `Object`s as Option<Arc<TObject>>

use std::{clone::Clone};
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
    pub inner: Option<Arc<TObject>>,
}

impl<'a> Object
where
    Object: TObject + 'a,
{
    pub fn new(obj: Option<Arc<TObject>>) -> Object {
        Object { inner: obj }
    }

    pub fn null() -> Object {
        Nil::new()
    }

    pub fn is_null(&self) -> bool {
        Object::isa::<Nil>(&self)
    }

    pub fn isa<T>(&'a self) -> bool
    where
        T: 'static,
    {
        let o = self.clone();
        match o.inner {
            None => false,
            Some(o) => {
                let a = CastArc::cast::<T>(o);
                match a {
                    Ok(_) => true,
                    _ => false,
                }
            }
        }
    }

    pub fn cast<T>(&'a self) -> Option<&'a T>
    where
        T: 'static,
    {
        let o = self.clone();
        match o.inner {
            None => None,
            Some(o) => {
                let a = CastArc::cast::<&T>(o);
                match a {
                    Ok(o) => {
                        let t = *o.as_ref();
                        Some(t)
                    },
                    _ => None,
                }
            },
        }
    }

    pub fn cast_mut<T>(&'a self) -> Option<&'a mut T>
    where
        T: 'static,
    {
        // let o = self.clone();
        // match o.inner {
        //     None => None,
        //     Some(o) => {
        //         let a = CastArc::cast::<T>(o);
        //         match a {
        //             Ok(o) => Some((*o.as_ref()).borrow_mut()),
        //             _ => None,
        //         }
        //     },
        // }
        todo!()
    }

    pub fn strong_count(&self) -> usize {
        let o = self.clone();
        match o.inner {
            None => 0,
            Some(o) => {
                Arc::<dyn TObject>::strong_count(&o)
            },
        }
    }

    pub fn call_by_id(&self, id: usize, args: &[Object]) -> Object {
        let o = self.clone();
        match o.inner {
            None => Object::new(None),
            Some(o) => {
                o.get_class().call(id, args).clone()
            },
        }
    }

    // pub fn call_by_name(&self, name: &str, args: &[Object]) -> Object {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.call(name, args).clone()
    // }

    pub fn get_by_id(&self, id: usize) -> Object {
        let a = self.clone();
        let b = a.get_class();
        b.get(id).clone()
    }

    // pub fn get_by_name(&self, name: &str) -> Object {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.get(name).clone()
    // }

    pub fn set_by_id(&self, id: usize, value: Object) -> Object {
        let a = self.clone();
        let b = a.get_class();
        b.set(id, value).clone()
    }

    // pub fn set_by_name(&self, name: &str, value: Object) -> Object {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.set(name, value).clone()
    // }
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
        let a = self.clone();
        match a.inner {
            None => todo!(),
            Some(o) => {o.get_class()},
        }
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
