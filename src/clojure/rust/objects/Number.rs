//! # All numbers and `Number` protocol
//!
//! Here will be all the number management, casting, operations
//! and functions.

use std::sync::*;

use clojure::rust::*;
// use clojure::lang::*;
use intertrait::*;

use crate::*;

/// All numeric values have the `Number` trait.
pub trait Number: IObject {
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

pub trait Floating {}

pub trait Decimal {}

pub trait Ratio {}

// use std::fmt::*;
number_def!(BigInteger, i128);
number_def!(Long, i64);
number_def!(Integer, i32);
number_def!(Short, i16);
number_def!(Byte, i8);
number_def!(Double, f64);
number_def!(Float, f32);
number_def!(Usize, usize);

#[test]
fn bidirectionnal_convert() {}
