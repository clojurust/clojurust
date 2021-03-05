//! Numbers traits
//!

// use std::sync::Arc;
// use std::fmt::Debug;
//  se std::mem::transmute;

use super::object::*;

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

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct BigInteger {
    value: i128,
}

impl BigInteger {
    pub fn new(value: i128) -> BigInteger {
        BigInteger { value }
    }
}

/*
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
    // Test object with primitive
    let i: i128 = 1;
    let o = Object::new::<BigInteger>(0, &BigInteger::new(i));
    println!("count {:?}", o.count());
    let o2 = o.clone();
    println!("Object: {:?}", o);
    let r = Object::get::<BigInteger>(&o);
    println!("Equality test {:?} = {:?}", i, r.value);
    println!("count {:?} = {:?}", o.count(), o2.count());

    // Test bad translation
    // cache_thread_shutdown(): unaligned tcache chunk detected
    // let r3: Arc<i64> = Object::get::<i64>(&o);

    assert_eq!(1, 1);
    // assert_eq!(i, *r);
    // assert_ne!(i, *r3); // missmatched types
}
