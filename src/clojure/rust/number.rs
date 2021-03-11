//! All numbers and `Number` protocol
//!

// use intertrait::cast::*;
use intertrait::*;

use crate::clojure;
use clojure::rust::class::*;
use clojure::rust::object::*;

/// All numeric values have the `Number` trait.
pub trait Number {
    fn big_integer_value(self) -> Object;

    fn long_value(self) -> Object;

    fn int_value(self) -> Object;

    fn short_value(self) -> Object;

    fn byte_value(self) -> Object;

    fn double_value(self) -> Object;

    fn float_value(self) -> Object;

    fn usize_value(self) -> Object;
}

pub trait Integer {}

pub trait Decimal {}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct BigInteger {
    value: i128,
}

castable_to!(BigInteger => [sync] TObject, Number);

impl BigInteger {
    pub fn new(value: i128) -> Object {
        Object::new::<BigInteger>(BigInteger { value })
    }
}

impl TObject for BigInteger {
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

impl Number for BigInteger {
    fn big_integer_value(self) -> Object {
        BigInteger::new(self.value as i128)
    }

    fn long_value(self) -> Object {
        Object::new(&Long::new(self.value as i64))
    }

    fn int_value(self) -> Object {
        Object::new(&Integer::new(self.value as i32))
    }

    fn short_value(self) -> Object {
        Object::new(&Short::new(self.value as i16))
    }

    fn byte_value(self) -> Object {
        Object::new(&Byte::new(self.value as i8))
    }

    fn double_value(self) -> Object {
        Object::new(&Double::new(self.value as f64))
    }

    fn float_value(self) -> Object {
        Object::new(&Float::new(self.value as f32))
    }

    fn usize_value(self) -> Object {
        todo!()
    }
}

pub unsafe fn init() {
    // only execute one time
    if INIT {
        return;
    }
    INIT = true;

    println!("Nil::init");

    // Insures all is initialized
    clojure::rust::object::init();
    clojure::rust::class::init();
}

static mut INIT: bool = false;

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
