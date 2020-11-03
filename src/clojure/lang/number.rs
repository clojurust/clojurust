//! Numbers traits 
//!

extern crate query_interface;

use query_interface::ObjectClone;
use super::object::CljObject;
use std::fmt::Debug;
use super::object::Object;

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

interfaces!(BigInteger: dyn CljObject, dyn Number, 
                        dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl BigInteger {
    pub fn new(value: i128) -> BigInteger {BigInteger {value,}}
}

impl CljObject for BigInteger {}

#[allow(dead_code)]
impl Number for BigInteger {
    fn big_integer_value(&self) -> Object {Object::new(&BigInteger::new(self.value as i128))}
    fn long_value(&self) -> Object {Object::new(&Long::new(self.value as i64))}
    fn int_value(&self) -> Object {Object::new(&Integer::new(self.value as i32))}
    fn short_value(&self) -> Object {Object::new(&Short::new(self.value as i16))}
    fn byte_value(&self) -> Object {Object::new(&Byte::new(self.value as i8))}
    fn double_value(&self) -> Object {Object::new(&Double::new(self.value as f64))}
    fn float_value(&self) -> Object {Object::new(&Float::new(self.value as f32))}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Long {
    value: i64,
}

interfaces!(Long: dyn CljObject, dyn Number, 
                  dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl Long {
    pub fn new(value: i64) -> Long {Long { value, }}
}

impl CljObject for Long {}

#[allow(dead_code)]
impl Number for Long {
    fn big_integer_value(&self) -> Object {Object::new(&BigInteger::new(self.value as i128))}
    fn long_value(&self) -> Object {Object::new(&Long::new(self.value as i64))}
    fn int_value(&self) -> Object {Object::new(&Integer::new(self.value as i32))}
    fn short_value(&self) -> Object {Object::new(&Short::new(self.value as i16))}
    fn byte_value(&self) -> Object {Object::new(&Byte::new(self.value as i8))}
    fn double_value(&self) -> Object {Object::new(&Double::new(self.value as f64))}
    fn float_value(&self) -> Object {Object::new(&Float::new(self.value as f32))}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Integer {
    value: i32,
}

interfaces!(Integer: dyn CljObject, dyn Number, dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl Integer {
    pub fn new(value: i32) -> Integer {Integer {value,}}
}

impl CljObject for Integer {}

#[allow(dead_code)]
impl<'a> Number for Integer {
    fn big_integer_value(&self) -> Object {Object::new(&BigInteger::new(self.value as i128))}
    fn long_value(&self) -> Object {Object::new(&Long::new(self.value as i64))}
    fn int_value(&self) -> Object {Object::new(&Integer::new(self.value as i32))}
    fn short_value(&self) -> Object {Object::new(&Short::new(self.value as i16))}
    fn byte_value(&self) -> Object {Object::new(&Byte::new(self.value as i8))}
    fn double_value(&self) -> Object {Object::new(&Double::new(self.value as f64))}
    fn float_value(&self) -> Object {Object::new(&Float::new(self.value as f32))}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Short {
    value: i16,
}

interfaces!(Short: dyn CljObject, dyn Number, dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl Short {
    pub fn new(value: i16) -> Short {Short {value,}}
}

impl CljObject for Short {}

#[allow(dead_code)]
impl Number for Short {
    fn big_integer_value(&self) -> Object {Object::new(&BigInteger::new(self.value as i128))}
    fn long_value(&self) -> Object {Object::new(&Long::new(self.value as i64))}
    fn int_value(&self) -> Object {Object::new(&Integer::new(self.value as i32))}
    fn short_value(&self) -> Object {Object::new(&Short::new(self.value as i16))}
    fn byte_value(&self) -> Object {Object::new(&Byte::new(self.value as i8))}
    fn double_value(&self) -> Object {Object::new(&Double::new(self.value as f64))}
    fn float_value(&self) -> Object {Object::new(&Float::new(self.value as f32))}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Byte {
    value: i8,
}

interfaces!(Byte: dyn CljObject, dyn Number, dyn ObjectClone, dyn Debug);

impl CljObject for Byte {}

#[allow(dead_code)]
impl Byte {
    pub fn new(value: i8) -> Byte {Byte {value,}}
}

#[allow(dead_code)]
impl Number for Byte {
    fn big_integer_value(&self) -> Object {Object::new(&BigInteger::new(self.value as i128))}
    fn long_value(&self) -> Object {Object::new(&Long::new(self.value as i64))}
    fn int_value(&self) -> Object {Object::new(&Integer::new(self.value as i32))}
    fn short_value(&self) -> Object {Object::new(&Short::new(self.value as i16))}
    fn byte_value(&self) -> Object {Object::new(&Byte::new(self.value as i8))}
    fn double_value(&self) -> Object {Object::new(&Double::new(self.value as f64))}
    fn float_value(&self) -> Object {Object::new(&Float::new(self.value as f32))}
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Double {
    value: f64,
}

interfaces!(Double: dyn CljObject, dyn Number, dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl Double {
    pub fn new(value: f64) -> Double {Double {value,}}
}

impl CljObject for Double {}

#[allow(dead_code)]
impl Number for Double {
    fn big_integer_value(&self) -> Object {Object::new(&BigInteger::new(self.value as i128))}
    fn long_value(&self) -> Object {Object::new(&Long::new(self.value as i64))}
    fn int_value(&self) -> Object {Object::new(&Integer::new(self.value as i32))}
    fn short_value(&self) -> Object {Object::new(&Short::new(self.value as i16))}
    fn byte_value(&self) -> Object {Object::new(&Byte::new(self.value as i8))}
    fn double_value(&self) -> Object {Object::new(&Double::new(self.value as f64))}
    fn float_value(&self) -> Object {Object::new(&Float::new(self.value as f32))}
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Float {
    value: f32,
}

interfaces!(Float: dyn CljObject, dyn Number, dyn ObjectClone, dyn Debug);
#[allow(dead_code)]
impl Float {
    pub fn new(value: f32) -> Float {Float {value,}}
}

impl CljObject for Float {}

#[allow(dead_code)]
impl Number for Float {
    fn big_integer_value(&self) -> Object {Object::new(&BigInteger::new(self.value as i128))}
    fn long_value(&self) -> Object {Object::new(&Long::new(self.value as i64))}
    fn int_value(&self) -> Object {Object::new(&Integer::new(self.value as i32))}
    fn short_value(&self) -> Object {Object::new(&Short::new(self.value as i16))}
    fn byte_value(&self) -> Object {Object::new(&Byte::new(self.value as i8))}
    fn double_value(&self) -> Object {Object::new(&Double::new(self.value as f64))}
    fn float_value(&self) -> Object {Object::new(&Float::new(self.value as f32))}
}

/*
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct BigDecimal {
    value: f128,
}

interfaces!(BigDecimal: dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl BigDecimal {
    pub fn new(value: f128) -> BigDecimal {BigDecimal {value,}}
}

impl Decimal for BigDecimal {}

#[allow(dead_code)]
impl Number for BigDecimal {
    fn big_integer_value(self) -> BigInteger {BigInteger::new(self.value as i128)}
    fn long_value(self) -> Long {Long::new(self.value as i64)}
    fn int_value(self) -> Integer {Integer::new(self.value as i32)}
    fn short_value(self) -> Short {Short::new(self.value as i16)}
    fn byte_value(self) -> Byte {Byte::new(self.value as i8)}
    fn double_value(self) -> Double {Double::new(self.value as f64)}
    fn float_value(self) -> Float {Float::new(self.value as f32)}
}
*/

