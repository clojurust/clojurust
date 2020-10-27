extern crate query_interface;

use query_interface::ObjectClone;
use std::fmt::Debug;

pub trait Numeric {}

pub trait Decimal {}

pub trait Number {
    fn long_value(self) -> Long;
    fn int_value(self) -> Integer;
    fn short_value(self) -> Short;
    fn byte_value(self) -> Byte;
    fn double_value(self) -> Double;
    fn float_value(self) -> Float;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Long {
    value: i64,
}

interfaces!(Long: dyn ObjectClone, dyn Debug, dyn Numeric);

#[allow(dead_code)]
impl Long {
    pub fn new(value: i64) -> Long {Long { value, }}
}

impl Numeric for Long {}

#[allow(dead_code)]
impl Number for Long {
    fn long_value(self) -> Long {Long::new(self.value as i64)}
    fn int_value(self) -> Integer {Integer::new(self.value as i32)}
    fn short_value(self) -> Short {Short::new(self.value as i16)}
    fn byte_value(self) -> Byte {Byte::new(self.value as i8)}
    fn double_value(self) -> Double {Double::new(self.value as f64)}
    fn float_value(self) -> Float {Float::new(self.value as f32)}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Integer {
    value: i32,
}

interfaces!(Integer: dyn ObjectClone, dyn Debug, dyn Numeric);

#[allow(dead_code)]
impl Integer {
    pub fn new(value: i32) -> Integer {Integer {value,}}
}

impl Numeric for Integer {}

#[allow(dead_code)]
impl Number for Integer {
    fn long_value(self) -> Long {Long::new(self.value as i64)}
    fn int_value(self) -> Integer {Integer::new(self.value as i32)}
    fn short_value(self) -> Short {Short::new(self.value as i16)}
    fn byte_value(self) -> Byte {Byte::new(self.value as i8)}
    fn double_value(self) -> Double {Double::new(self.value as f64)}
    fn float_value(self) -> Float {Float::new(self.value as f32)}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Short {
    value: i16,
}

interfaces!(Short: dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl Short {
    pub fn new(value: i16) -> Short {Short {value,}}
}

impl Numeric for Short {}

#[allow(dead_code)]
impl Number for Short {
    fn long_value(self) -> Long {Long::new(self.value as i64)}
    fn int_value(self) -> Integer {Integer::new(self.value as i32)}
    fn short_value(self) -> Short {Short::new(self.value as i16)}
    fn byte_value(self) -> Byte {Byte::new(self.value as i8)}
    fn double_value(self) -> Double {Double::new(self.value as f64)}
    fn float_value(self) -> Float {Float::new(self.value as f32)}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Byte {
    value: i8,
}

interfaces!(Byte: dyn ObjectClone, dyn Debug);

impl Numeric for Byte {}

#[allow(dead_code)]
impl Byte {
    pub fn new(value: i8) -> Byte {Byte {value,}}
}

#[allow(dead_code)]
impl Number for Byte {
    fn long_value(self) -> Long {Long::new(self.value as i64)}
    fn int_value(self) -> Integer {Integer::new(self.value as i32)}
    fn short_value(self) -> Short {Short::new(self.value as i16)}
    fn byte_value(self) -> Byte {Byte::new(self.value as i8)}
    fn double_value(self) -> Double {Double::new(self.value as f64)}
    fn float_value(self) -> Float {Float::new(self.value as f32)}
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Double {
    value: f64,
}

interfaces!(Double: dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl Double {
    pub fn new(value: f64) -> Double {Double {value,}}
}

impl Decimal for Double {}

#[allow(dead_code)]
impl Number for Double {
    fn long_value(self) -> Long {Long::new(self.value as i64)}
    fn int_value(self) -> Integer {Integer::new(self.value as i32)}
    fn short_value(self) -> Short {Short::new(self.value as i16)}
    fn byte_value(self) -> Byte {Byte::new(self.value as i8)}
    fn double_value(self) -> Double {Double::new(self.value as f64)}
    fn float_value(self) -> Float {Float::new(self.value as f32)}
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Float {
    value: f32,
}

interfaces!(Float: dyn ObjectClone, dyn Debug);

#[allow(dead_code)]
impl Float {
    pub fn new(value: f32) -> Float {Float {value,}}
}

impl Decimal for Float {}

#[allow(dead_code)]
impl Number for Float {
    fn long_value(self) -> Long {Long::new(self.value as i64)}
    fn int_value(self) -> Integer {Integer::new(self.value as i32)}
    fn short_value(self) -> Short {Short::new(self.value as i16)}
    fn byte_value(self) -> Byte {Byte::new(self.value as i8)}
    fn double_value(self) -> Double {Double::new(self.value as f64)}
    fn float_value(self) -> Float {Float::new(self.value as f32)}
}

use f128;

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
    fn big_decimal_value(self) -> BigDecimal {BigDecimal::new(self.value as f128)}
    fn double_value(self) -> Double {Double::new(self.value as f64)}
    fn float_value(self) -> Float {Float::new(self.value as f32)}
}

