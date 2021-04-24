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

// init_obj! {
//     Object {
//         clojure::rust::Class::init();
//         clojure::rust::IObject::init();
//         clojure::rust::ObjResult::init();
//     }
// }

pub struct Object
{
    pub inner: Option<Arc<IObject>>,
}

// #[derive(Debug)]
// pub struct Object {
//     pub inner: Option<Arc<IObject>>,
// }

impl<'a> Object
where
    Object: IObject+'a,
{
    pub fn new(obj: Option<Arc<IObject>>) -> Object
    {
        Object {
            inner: obj,
        }
    }

    pub fn null() -> Object
    {
        Object {
            inner: None,
        }
    }

    pub fn is_null(&self) -> bool
    {
        match self.inner
        {
            | Some(_) => false,
            | None => true,
        }
    }

    pub fn isa<T>(&'a self) -> bool
    where
        T: 'static,
    {
        let o = self.clone();
        match o.inner
        {
            | None => false,
            | Some(o) =>
            {
                let a = CastArc::cast::<T>(o).as_deref();

                match a
                {
                    | Ok(_) => true,
                    | _ => false,
                }
            },
        }
    }

    pub fn cast<T>(&'a self) -> ObjResult<&'static T>
    {
        match self.inner
        {
            | None => err_cast(self, "?"),
            | Some(o) =>
            {
                let a = CastArc::cast::<T>(o).as_deref();
                match a
                {
                    | Ok(o) => Ok(o),
                    | Err(_) => err_cast(self, "?"),
                }
            },
        }
    }

    pub fn strong_count(&self) -> usize
    {
        let o = self.clone();
        match o.inner
        {
            | None => 0,
            | Some(o) => Arc::<dyn IObject>::strong_count(&o),
        }
    }

    pub fn call(
        &self,
        id: usize,
        args: &[Object],
    ) -> ObjResult<Object>
    {
        let o = self.clone();
        match o.inner
        {
            | None => err("Cannot call function on nil"),
            | Some(o) => o.getClass().call(self.clone(), id, args),
        }
    }

    // pub fn call_by_name(&self, name: &str, args: &[Object]) ->
    // ObjError<Object> {     let a = self.clone();
    //     let b = a.get_class();
    //     b.call(name, args).clone()
    // }

    pub fn get(
        &self,
        id: usize,
    ) -> ObjResult<Object>
    {
        let a = self.clone();
        let b = a.getClass();
        b.get(self.clone(), id)
    }

    // pub fn get_by_name(&self, name: &str) -> ObjError<Object> {
    //     let a = self.clone();
    //     let b = a.get_class();
    //     b.get(name).clone()
    // }
}

/// SImplementation of protocol IObject for Object.
///
/// Functions are applied to the `content` of `Object`
// #[cast_to([sync] IObject, Debug)];
impl IObject for Object
{
    fn getClass<'a>(&self) -> &'a SClass
    {
        let a = self.clone();
        match a.inner
        {
            | None => todo!(),
            | Some(o) => o.getClass(),
        }
    }

    fn hashCode(&self) -> usize
    {
        match self.inner
        {
            | Some(o) => o.hashCode(),
            | None => 0,
        }
    }

    fn equals(
        &self,
        other: &Object,
    ) -> bool
    {
        match self.inner
        {
            | Some(o) => o.equals(other),
            | None => match other.inner
            {
                | Some(_) => false,
                | None => true,
            },
        }
    }

    fn toString(&self) -> String { todo!() }
}

impl Clone for Object
{
    fn clone(&self) -> Self
    {
        let Object {
            inner,
        } = self;
        Object {
            inner: inner.clone(),
        }
    }
}

impl Eq for Object {}

impl PartialEq for Object
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.equals(other)
    }
}

impl Hash for Object
{
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    )
    {
        state.write_usize(self.hashCode())
    }

    fn hash_slice<H: Hasher>(
        data: &[Self],
        state: &mut H,
    ) where
        Self: Sized,
    {
        for piece in data
        {
            piece.hash(state);
        }
    }
}
