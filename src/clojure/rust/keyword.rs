//! clojure::rust::keyword: keyword index

use im::vector::Vector;
use im::hashmap::HashMap;
use ::lazy_static::lazy_static;
use ::mut_static::MutStatic;
use std::*;
//use std::io::prelude::*;

lazy_static! {
    static ref KEYWORDS: MutStatic<Vector<&'static str>> = 
            MutStatic::new();
    static ref KEY_MAP: MutStatic<HashMap<&'static str, usize>> = 
            MutStatic::new();
}
#[allow(dead_code)]
fn init_static() {
    println!("init static");
    KEYWORDS.set(Vector::<&str>::new()).unwrap();
    KEY_MAP.set(HashMap::<&str, usize>::new()).unwrap(); 
}

#[allow(dead_code)]
fn add_keyword(keyword: &'static str) -> usize {
    println!("Add keyword");
    if let Some(index) = KEY_MAP.read().unwrap().get(keyword) {
        println!("Already exists");
        *index
    }
    else {
        println!("Don't exists");
        let index = KEYWORDS.read().unwrap().len();
        println!("len = {}", index);
        let mut mut_handle = KEY_MAP.write().unwrap();
        mut_handle.insert(keyword, index);
        index
    }
}

#[test]
fn test_the_thing() -> io::Result<()> {
    println!("Go init_static");
    init_static(); // expected to succeed
    println!("Go add_keyword");
    let a = add_keyword("fdsafdsafdsa");
    println!("index = {}", a);
    Ok(())
}