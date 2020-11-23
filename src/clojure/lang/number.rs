//! Numbers traits 
//!

use std::{sync::Arc, mem::transmute};
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
impl BigInteger
{
    pub fn new_i128(value: i128) -> Arc<Object> {
        unsafe {
            Arc::new(Object {
                class: 3,
                ptr: transmute::<*const i128, usize>(Arc::into_raw(Arc::new(value))),
            })
        }
    }

    pub fn new(value: BigInteger) -> Arc<Object> {
        unsafe {
            Arc::new(Object {
                class: 4,
                ptr: transmute::<*const BigInteger, usize>(Arc::into_raw(Arc::new(value))),
            })
        }
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
    // Test object with primitive
    let i: i128 = 432143214321432143214;
    let o = BigInteger::new_i128(i);
    println!("{:?}", o);
    let r = Object::get::<i128>(&o);
    println!("{} = {}", i, r);

    // Test object with structure
    let i2: BigInteger = BigInteger{value: i};

    let o2: Arc<Object> = BigInteger::new(i2);
    let r2: Arc<BigInteger> = Object::get::<BigInteger>(&o2);
    println!("{:?} = {:?}", i2, r2);

    // Test bad translation
    // cache_thread_shutdown(): unaligned tcache chunk detected
    // let r3: Arc<i64> = Object::get::<i64>(&o);

    assert_eq!(i, *r);
    assert_eq!(i2, *r2);
    // assert_ne!(i, *r3); // missmatched types
}
