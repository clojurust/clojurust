//! # All numbers and `Number` protocol
//!
//! Here will be all the number management, casting, operations
//! and functions.

use std::sync::*;

use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

/// All numeric values have the `Number` trait.
pub trait Number: CastFromSync {
    fn big_integer_value_o(&self) -> Object;

    fn long_value_o(&self) -> Object;

    fn int_value_o(&self) -> Object;

    fn short_value_o(&self) -> Object;

    fn byte_value_o(&self) -> Object;

    fn double_value_o(&self) -> Object;

    fn float_value_o(&self) -> Object;

    fn usize_value_o(&self) -> Object;

    fn big_integer_value(&self) -> i128;

    fn long_value(&self) -> i64;

    fn int_value(&self) -> i32;

    fn short_value(&self) -> i16;

    fn byte_value(&self) -> i8;

    fn double_value(&self) -> f64;

    fn float_value(&self) -> f32;

    fn usize_value(&self) -> usize;
}

pub trait Numeric {}

pub trait Decimal {}

#[macro_use]
use crate::number_def;

number_def!(BigInteger, BigInteger, i128);
number_def!(Long, Long, i64);
number_def!(Integer, Integer, i32);
number_def!(Short, Short, i16);
number_def!(Byte, Byte, i8);
number_def!(Double, Double, f64);
number_def!(Float, Float, f32);
number_def!(Usize, Usize, usize);

static mut INIT: bool = false;

#[test]
fn bidirectionnal_convert() {
    // Test object with primitive
    let i: i128 = 1;
    let o = BigInteger::new(i);
    println!("count {:?}", o.count());
    let o2 = o.clone();
    println!("Object: {:?}", o);

    let t = Object::isa::<BigInteger>(&o);
    let t2 = o.inner.type_id();

    // !! doesn't work cast to unimplemented trait TObject
    // let r = Object::inn::<Number>(&mut o);

    // doesn't work directly neither
    // let s: &dyn Number = &o;

    // println!("Equality test {:?} = {:?}", i, r.big_integer_value());
    println!("count {:?} = {:?}", o.count(), o2.count());

    // Test translation
    // Object should be passed to a &mut... tbt
    let r3 = Object::inn::<Integer>(&mut o);

    // assert_eq!(i, Object::new::<Number>(*r3));
    // assert_ne!(i, *r3); // mismatched types
}
