//! Anonymous Function with one defined arity, and possible last element multi arity

use super::object::*;
use super::pvector::*;
use std::sync::*;

pub struct SImplementation {
    pub multiary: bool,
    pub function: FnPtr,
}

type FnPtr = fn(args: &Object) -> Object;

pub trait Implementation {
    fn call(&self, args: &Object) -> Object;
}
impl SImplementation {
    fn new(multiary: bool, function: FnPtr) -> Object {
        Object::new::<SImplementation>(SImplementation { multiary, function })
    }
}

impl Implementation for SImplementation {
    fn call(&self, args: &Object) -> Object {
        let f = self.function;
        f(args)
    }
}

impl TObject for SImplementation {
    fn get_class(&self) -> &super::class::SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: usize) -> Object {
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
}

pub fn init() {}
