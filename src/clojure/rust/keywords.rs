//! clojure::rust::keyword: keyword index

use im::*;
use im::hashmap::HashMap;
use std::sync::*;
use lazy_static::lazy_static;

/// A keyword storage structure
///
/// We will store all `String`s used as reference to objects as `usize`.
/// `usize` values are unique and immutable for every `String`.
/// `Strings` are added incrementally to the `vect` `Vector` and cannot be destroyed.
/// as a `String` is added, it's index is added in the `map` `HashMap`.
/// # Exxamples
/// ```
///
#[derive(Default)]
#[derive(Debug)]
pub struct Keywords {
    map: HashMap<String, usize>,
    vect: Vector<String>,
}

impl Keywords {
    pub fn new() -> Keywords {
        Keywords {
            map: HashMap::<String, usize>::new(),
            vect: Vector::<String>::new(),
        }
    }

    pub fn current() -> Arc<Keywords> {
        KEYWORDS.read().unwrap().clone()
    }

    pub fn current_mut() -> Arc<Keywords> {
        KEYWORDS.write().unwrap().clone()
    }

    pub fn make_current(self) {
        println!("make_current({:?})", self);
        *KEYWORDS.write().unwrap() = Arc::new(self);
    }

    pub fn len() -> usize {
        println!("len()");
        Keywords::current().vect.len()
    }

    pub fn get(key: String) -> usize {
        let i = Keywords::current().clone();
        let a = i.as_ref();
        let m = a.map.as_ref();
        let mut v = a.vect.clone();
        let length = Keywords::len();

        match m.get(&key) {
            // found entry
            Some(idx) => { *idx }

            // Not found: add entry in vect and map
            None => {
                // Insert new values in vector and map
                &v.push_back(key.clone()); 
                let k = Keywords {
                    map: m.update(key.clone(), length.clone()),
                    vect: v,
                };
                k.make_current();

                // return new index that was length of vector
                length
            }
        }
    }

    pub fn to_string(index: usize) -> String {
        match Keywords::current().vect.get(index) {
            Some(key) => { String::from(key) }
            None => {String::from("")}
        }
    }

    pub fn init_static() {
    }
}

lazy_static! {
    /// Private access to static global `Keywords` struture.
    ///
    /// Here will be stored and retrived keywords data.
    pub static ref KEYWORDS: RwLock<Arc<Keywords>> = 
                        RwLock::new(Arc::new(Default::default()));
}

#[test]
fn test_the_thing() {
    println!("Init state {:?}", Keywords::len());
    Keywords::init_static(); // expected to succeed
    println!("New state {:?}", Keywords::len());
    let o = Keywords::get(String::from("essai"));
    println!("add essai {:?} {}", Keywords::len(), 
                                    Keywords::to_string(0));
    Keywords::get("essai2".to_string());
    println!("add essai2 {:?} {} {}", Keywords::len(), 
                                    Keywords::to_string(0),
                                    Keywords::to_string(1));
    
    println!("Keyword 0 = \"{}\"", Keywords::to_string(0));
    println!("Keyword 1 = \"{}\"", Keywords::to_string(1));
    println!("Keyword 2 = \"{}\"", Keywords::to_string(2));
    assert_eq!(1,1)
}
