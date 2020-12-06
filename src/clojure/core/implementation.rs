//! Anonymous Function with one defined arity, and possible last element multi arity

use std::sync::*;
use super::object::*;

#[derive(Clone)]
pub struct Implementation
{
    pub multiary: bool,
    pub function: &'static dyn Fn(&[Arc<Object>]) -> Arc<Object>,
}

impl Implementation 
{
        pub fn new(multiary: bool, function: &'static dyn Fn(&[Arc<Object>]) -> Arc<Object>) -> Implementation {
        Implementation {
            multiary,
            function,
        }
    }

        pub fn call(&self, args: &[Arc<Object>]) -> Arc<Object> {
        self.function.call((args,))
    }
}

pub fn init() {

}

////////////////////////////////////////////////////////
// Tests

/***************************
// This doesn'd work, but compiles
fn func(args: &[Arc<Object>]) -> Arc<Object> {
    println!("before func clone: {}", Arc::strong_count(&args[0]));
    let res = args[0].clone();
    println!("after func clone: {}", Arc::strong_count(&args[0]));
    res
}

fn func2() -> (Arc<Object>, Arc<Object>){
    let args: &[Arc<Object>] = &[Object::new(0, 214321i128)];
    let i = Implementation::new(false, &func);
    println!("before func call: {}", Arc::strong_count(&args[0]));
    let r = i.call(args);
    println!("after func call: {}", Arc::strong_count(&r));
    let res = (args[0].clone(), r.clone());
    println!("after 2* clone for result: {}", Arc::strong_count(&r));
    res
}

#[test]
fn test_function_call() {
    // a and b references args of func 2, droping at the end of the function
    // makes the memory management fails miserably... :D
    let (a , b) = func2();
    println!("after func2 call: a={} b={}", Arc::strong_count(&a), Arc::strong_count(&b));
    println!("arguments {:?} resultat {:?}", Object::get::<i128>(a.as_ref()), Object::get::<i128>(b.as_ref()));
    assert_eq!(Object::get::<i128>(a.as_ref()), Object::get::<i128>(b.as_ref()));
    // but until here, there's no problem... ;)
}
*/

