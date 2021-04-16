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
            None => None,
            Some(o) => {
                let a = CastArc::cast::<T>(o).as_deref();
                match a {
                    Ok(o) => {
                        Ok(o)
                    },
                    
                    Err(_) => {}
                }
            },
        }
    }

    pub fn cast_mut<T>(&'a self) -> ObjResult<&'a mut T>
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

    pub fn call_by_id(&self, id: usize, args: &[Object]) -> ObjError<Object> {
        let o = self.clone();
        match o.inner {
            None => Object::new(None),
            Some(o) => {
                o.get_class().call(id, args).clone()
            },
        }
    }

    // pub fn call_by_name(&self, name: &str, args: &[Object]) -> ObjError<Object> {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.call(name, args).clone()
    // }

    pub fn get_by_id(&self, id: usize) -> ObjError<Object> {
        let a = self.clone();
        let b = a.get_class();
        b.get(id).clone()
    }

    // pub fn get_by_name(&self, name: &str) -> ObjError<Object> {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.get(name).clone()
    // }

    pub fn set_by_id(&self, id: usize, value: Object) -> ObjError<Object> {
        let a = self.clone();
        let b = a.get_class();
        b.set(id, value).clone()
    }

    // pub fn set_by_name(&self, name: &str, value: Object) -> ObjError<Object> {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.set(name, value).clone()
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
        let a = self.clone();
        a.get_class().get_hash(a)
    }

    fn equals(&self, other: &Object) -> bool {
        let a = self.clone();
        a.get_class().equals(other)
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
