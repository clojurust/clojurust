//! # Defines library's dynamic Strings.
//!

use std::{convert::*, fmt::*};

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::IObject;
    clojure::rust::Class;
}

castable_to!(String => [sync] IObject, IString);

init_obj! {
    Stri {
        clojure::rust::Object::init();
        clojure::rust::IObject::init();
        clojure::rust::Class::init();
    }
}

// #[derive(Debug)]
// pub struct SStri {
//     pub inner: String,
// }

pub trait IString: IObject {}

impl String {}

impl IString for String {}

/// String -> Object
impl From<String> for Object {
    fn from(s: String) -> Self {
        Object::from(String::from(s))
    }
}

/// &str -> Object
impl From<&str> for Object {
    fn from(s: &str) -> Self {
        Object::from(String::from(s))
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\"{:?}\"", self.inner)
    }
}

impl IObject for String {
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
