//! Anonymous Function with one defined arity, and possible last element multi arity
// use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;

type FnPtr = fn(args: &Object) -> Object;

pub struct SImplemNative {
    pub multiary: bool,
    pub function: FnPtr,
}

castable_to!(SImplemNative => [sync] TObject, ImplemNative);

pub trait ImplemNative {
    fn call(&self, args: &Object) -> Object;

    fn call_multi_arity(&self, multi: usize, args: &Object) -> Object;
}

impl SImplemNative {
    fn new(multiary: bool, function: FnPtr) -> Object {
        Object::new::<SImplemNative>(SImplemNative { multiary, function })
    }
}

impl ImplemNative for SImplemNative {
    fn call(&self, args: &Object) -> Object {
        let f = self.function;
        f(args)
    }

    fn call_multi_arity(&self, multi: usize, args: &Object) -> Object {
        let f = self.function;
        f(args)
    }
}

impl TObject for SImplemNative {
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
