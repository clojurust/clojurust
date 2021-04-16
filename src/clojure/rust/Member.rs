//! # Access to `Member`s getters and setters of `Objects` inner structure.
//!
//! This will be part of the `Class` store.

// use intertrait::cast::*;

use std::fmt::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
}

castable_to!(SMember => [sync] TObject, Member);

init_obj! {
    Member {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}

#[derive(Debug)]
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

impl Display for SMember {
    /// Return string representation of
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Member {}", self.name)
    }
}

impl TObject for SMember {
    fn get_class<'a>(&self) -> &'a SClass {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}
