//! Anonymous Function with one defined arity
//! Function can be with multi-arity according to SFunction.multiarity.

// use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;

pub type SFnNative = fn(args: &Object) -> Object;

castable_to!(SFnNative => [sync] TObject, FnNative);

pub trait FnNative {
    fn call(&self, args: &Object) -> Object;
}

impl FnNative {
    fn new(function: SFnNative) -> Object {
        Object::new::<SFnNative>(function)
    }
}

impl FnNative for SFnNative {
    fn call(&self, args: &Object) -> Object {
        let f = self;
        f(args)
    }
}

impl TObject for SFnNative {
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
}

pub fn init() {}
