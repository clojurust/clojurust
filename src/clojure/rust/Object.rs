//! # Defines Rust dynamic `Object`s.
//!
//! Define dynamic `Object`s as Option<Arc<TObject>>

use std::{clone::Clone};
use std::{fmt::*, hash::*, sync::*};

use intertrait::cast::CastArc;

use crate::*;

use_obj! {
    clojure::rust::Class;
    clojure::rust::ObjError;
}

castable_to!(Object => [sync] TObject);

init_obj! {
    Object {
        clojure::rust::Class::init();
    }
}

#[derive(Debug)]
pub struct Object {
    pub inner: Option<Arc<TObject>>,
}

/// `Object` `Protocol` for all defined `Object`s
///
///
pub trait TObject: Debug + Display + CastFromSync  {
    /// Return `Class` of `Object`
    fn get_class<'a>(&self) -> &'a SClass;

    fn get_hash(&self) -> usize;

    fn equals(&self, other: &Object) -> bool;
}

impl<'a> Object
where
    Object: TObject + 'a,
{
    pub fn new(obj: Option<Arc<TObject>>) -> Object {
        Object { inner: obj }
    }

    pub fn null() -> Object {
        Object { inner: None }
    }

    pub fn is_null(&self) -> bool {
        match self.inner {
            Some(_) => {false}
            None => {true}
        }
    }

    pub fn isa<T>(&'a self) -> bool
    where
        T: 'static,
    {
        let o = self.clone();
        match o.inner {
            None => false,
            Some(o) => {
                let a = CastArc::cast::<T>(o).as_deref();
                match a {
                    Ok(_) => true,
                    _ => false,
                }
            }
        }
    }

    pub fn cast<T>(&'a self) -> ObjResult<&'static T>
    {
        match self.inner {
            None => err_cast(self,"?"),
            Some(o) => {
                let a = CastArc::cast::<T>(o).as_deref();
                match a {
                    Ok(o) => {
                        Ok(o)
                    },
                    Err(_) => err_cast(self,"?")
                }
            },
        }
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

    pub fn call_by_id(&self, id: usize, args: &[Object]) -> ObjResult<Object> {
        let o = self.clone();
        match o.inner {
            None => err("Cannot call function on nil"),
            Some(o) => {
                o.get_class().call(self.clone(), id, args)
            },
        }
    }

    // pub fn call_by_name(&self, name: &str, args: &[Object]) -> ObjError<Object> {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.call(name, args).clone()
    // }

    pub fn get_by_id(&self, id: usize) -> ObjResult<Object> {
        let a = self.clone();
        let b = a.get_class();
        b.get(self.clone(), id)
    }

    // pub fn get_by_name(&self, name: &str) -> ObjError<Object> {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.get(name).clone()
    // }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.inner {
            Some(inner) => {write!(f, "{:?}", inner)}
            None => {write!(f, "nil")}
        }
    }
}

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

    fn get_hash(&self) -> usize {
        match self.inner {
            Some(o) => {o.get_hash()}
            None => {0}
        }
    }

    fn equals(&self, other: &Object) -> bool {
        match self.inner {
            Some(o) => {o.equals(other)}
            None => {
                match other.inner {
                    Some(_) => {false}
                    None => {true}
                }
            }
        }
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
