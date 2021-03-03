//! Anonymous Function with one defined arity, and possible last element multi arity
use std::sync::*;

use intertrait::cast::*;
use intertrait::*;

use super::object;

pub struct SImplementation {
    pub multiary: bool,
    pub function: FnPtr,
}

castable_to!(SImplementation => [sync] object::TObject, Implementation);

type FnPtr = fn(args: &object::Object) -> object::Object;

pub trait Implementation {
    fn call(&self, args: &object::Object) -> object::Object;
}

impl SImplementation {
    fn new(multiary: bool, function: FnPtr) -> object::Object {
        object::Object::new::<SImplementation>(SImplementation { multiary, function })
    }
}

impl Implementation for SImplementation {
    fn call(&self, args: &object::Object) -> object::Object {
        let f = self.function;
        f(args)
    }
}

impl object::TObject for SImplementation {
    fn get_class(&self) -> &super::class::SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[object::Object]) -> object::Object {
        todo!()
    }

    fn get(&self, name: usize) -> object::Object {
        todo!()
    }

    fn to_string(&self) -> &str {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &object::Object) -> bool {
        todo!()
    }
}

pub fn init() {}
