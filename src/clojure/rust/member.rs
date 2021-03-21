//! # Access to `Member`s getters and setters of `Objects` inner structure.
//!
//! This will be part of the `Class` store.

// use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::object;
    clojure::rust::class;
}

castable_to!(SMember => [sync] TObject, Member);

init_obj! {
    Member {
        clojure::rust::object::init();
        clojure::rust::class::init();
    }
}

pub struct SMember {
    name: usize,
    class: usize,
    getter: Object,
    setter: Object,
}

impl SMember {
    pub fn new(
        name: usize,
        class: usize,
        getter: Object, // function ?
        setter: Object, // function ?
    ) -> SMember {
        SMember {
            name,
            class,
            getter,
            setter,
        }
    }
}

pub trait Member: CastFromSync {}

impl Member {}

impl Member for SMember {}

impl TObject for SMember {
    fn get_class<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn to_string(&self) -> &str {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }

    fn isa<T>(obj: &Object) -> bool
    where
        T: TObject + ?Sized + 'static,
        Self: Sized,
    {
        todo!()
    }

    fn inn<T>(obj: &Object) -> &T
    where
        T: TObject + ?Sized + 'static,
        Self: Sized,
    {
        todo!()
    }

    fn inn_mut<T>(obj: &Object) -> &mut T
    where
        T: TObject + ?Sized + 'static,
        Self: Sized,
    {
        todo!()
    }
}
