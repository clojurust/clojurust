//! # Defines Rust dynamic `Object`s.
//!
//! Define dynamic `Object`s as Option<Arc<IObject>>

use std::clone::Clone;
use std::hash::*;
use std::sync::*;

use clojure::rust::*;
use intertrait::cast::CastArc;
use intertrait::*;

use crate::*;
castable_to!(Object => [sync] IObject);

pub struct Object {
    pub inner: Arc<IObject>,
}

impl<'a> Object
where
    Object: IObject+'a,
{
    pub fn new(obj: Arc<IObject>) -> Object {
        Object {
            inner: obj,
        }
    }

    pub fn null() -> Object {
        // Object {
        //     inner: None,
        // }
    }

    pub fn is_null(&self) -> bool {
        match self.inner {
            | Some(_) => false,
            | None => true,
        }
    }

    pub fn isa<T>(&'a self) -> bool
    where
        T: 'static,
    {
        let o = self.clone();
        match o.inner {
            | None => false,
            | Some(o) => {
                let obj = CastArc::cast::<T>(o);
                let a = obj.as_deref();

                match a {
                    | Ok(_) => true,
                    | _ => false,
                }
            },
        }
    }

    pub fn cast<T>(&'a self) -> ObjResult<Arc<T>>
    where
        T: 'static,
    {
        let o = self.clone();
        match o.inner {
            | None => err_cast(&self, "?"),
            | Some(o) => {
                let obj = CastArc::cast::<T>(o);
                match obj {
                    | Ok(b) => Ok(b),
                    | Err(e) => err_cast(self, "T"),
                }
            },
        }
    }

    pub fn strong_count(&self) -> usize {
        match &self.inner {
            | None => 0,
            | Some(o) => Arc::<dyn IObject>::strong_count(&o),
        }
    }

    pub fn call(
        &self,
        id: usize,
        args: &[Object],
    ) -> ObjResult<Object> {
        let o = self.clone();
        match o.inner {
            | None => err("Cannot call function on nil"),
            | Some(o) => o.getClass().call(self.clone(), id, args),
        }
    }

    pub fn get(
        &self,
        id: usize,
    ) -> ObjResult<Object> {
        let a = self.clone();
        let b = a.getClass();
        b.get(self.clone(), id)
    }
}

/// SImplementation of protocol IObject for Object.
///
/// Functions are applied to the `content` of `Object`
// #[cast_to([sync] IObject, Debug)];
impl IObject for Object {
    fn getClass<'a>(&self) -> &'a SClass {
        let o = self.clone();
        match o.inner {
            | None => todo!(),
            | Some(o) => o.getClass(),
        }
    }

    fn hashCode(&self) -> usize {
        let o = self.clone();
        match o.inner {
            | Some(o) => o.hashCode(),
            | None => 0,
        }
    }

    fn equals(
        &self,
        other: &Object,
    ) -> bool {
        let o = self.clone();
        match o.inner {
            | Some(o) => o.equals(other),
            | None => match other.inner {
                | Some(_) => false,
                | None => true,
            },
        }
    }

    fn toString(&self) -> String {
        let o = self.clone();
        match o.inner {
            | Some(o) => o.toString(),
            | None => String::from("nil"),
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        let Object {
            inner,
        } = self;
        Object {
            inner: inner.clone(),
        }
    }
}

impl Eq for Object {}

impl PartialEq for Object {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.equals(other)
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        state.write_usize(self.hashCode())
    }

    fn hash_slice<H: Hasher>(
        data: &[Self],
        state: &mut H,
    ) where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state);
        }
    }
}
