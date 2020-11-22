//! Numbers traits 
//!

use std::fmt::Debug;
use crate::clojure::rust::object::Object;
//  se std::mem::transmute;

/// All numeric values have the `Number` trait.
pub trait Number {
    fn big_integer_value(&self) -> Object;
    fn long_value(&self) -> Object;
    fn int_value(&self) -> Object;
    fn short_value(&self) -> Object;
    fn byte_value(&self) -> Object;
    fn double_value(&self) -> Object;
    fn float_value(&self) -> Object;
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct BigInteger {
    value: i128,
}

#[allow(dead_code)]
impl<'i> BigInteger {
    pub fn new(value: i128) -> &'i Object<'i> {
        &Object::new(1,
                     &BigInteger{value})
    }
}

/*
#[allow(dead_code)]
pub impl Number for BigInteger {
    fn big_integer_value(&self) -> Object {Object::new(&BigInteger::new(*self.value as &i128))}
    fn long_value(&self) -> Object {Object::new(&Long::new(self.value as i64))}
    fn int_value(&self) -> Object {Object::new(&Integer::new(self.value as i32))}
    fn short_value(&self) -> Object {Object::new(&Short::new(self.value as i16))}
    fn byte_value(&self) -> Object {Object::new(&Byte::new(self.value as i8))}
    fn double_value(&self) -> Object {Object::new(&Double::new(self.value as f64))}
    fn float_value(&self) -> Object {Object::new(&Float::new(self.value as f32))}
}
*/

#[test]
fn bidirectionnal_convert() {
    // let f = BigInteger::big_integer_value;
    let i: i128 = 0;
    let o = BigInteger::new(i);
    let r = Object::get::<i128>(o);
    assert_eq!(i, *r);
}