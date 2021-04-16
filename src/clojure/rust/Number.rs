//! # All numbers and `Number` protocol
//!
//! Here will be all the number management, casting, operations
//! and functions.

use std::sync::*;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
}

castable_to!(BigInteger => [sync] TObject, Number);
castable_to!(Long => [sync] TObject, Number);
castable_to!(Integer => [sync] TObject, Number);
castable_to!(Short => [sync] TObject, Number);
castable_to!(Byte => [sync] TObject, Number);
castable_to!(Double => [sync] TObject, Number);
castable_to!(Float => [sync] TObject, Number);
castable_to!(Usize => [sync] TObject, Number);

init_obj! {
    Number {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}

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

pub trait Floating {}

pub trait Decimal {}

pub trait Ratio {}

use crate::number_def;
use std::fmt::*;
number_def!(BigInteger, i128);
number_def!(Long, i64);
number_def!(Integer, i32);
number_def!(Short, i16);
number_def!(Byte, i8);
number_def!(Double, f64);
number_def!(Float, f32);
number_def!(Usize, usize);

#[test]
fn bidirectionnal_convert() {
    // Test object with primitive
    // use std::any::TypeId;

    use crate::clojure;
    use clojure::rust::Number::*;
    // use clojure::rust::Object::*;

    let nil: Object = Object::null();

    let number: Object = BigInteger::new(1);
    println!("count at begining={:?}", number.strong_count());

    let number2: Object = number.clone();
    println!("count after clone={:?}", number.strong_count());

    println!("original Object: {:?}", &number);
    println!("cloned Object: {:?}", &number2);

    // cast Object BigInteger -> Protocol Number
    if let Some(number_as_number) = number.cast::<&dyn Number>() {
        print!("number_as_number = {}", number_as_number.double_value());
    }

    // cast Object Nil -> Protocol Number
    if let Some(nil_as_number) = nil.cast::<&dyn Number>() {
        print!(
            "nil_as_number = {} is a number",
            nil_as_number.double_value()
        );
    } else {
        print!("nil_as_number = {} is not a number", nil.to_string());
    }

    // Cast Object Nil -> Boxed Nil struct
    // if let Some(nil_as_struct_nil) = nil.cast::<Nil>() {
    //     print!("Boxed value if Nil Object: {:?}", nil_as_struct_nil);
    // }

    // let type_id: TypeId = number.type_id();
}
