//! clojure::rust::keyword: keyword index

use im::vector::Vector;
use im::hashmap::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
struct Keywords {
    map: HashMap<&'static str, usize>,
    vect: Vector<&'static str>,
}

impl Keywords {
    fn new() -> Keywords {
        Keywords {
            map: HashMap::<&'static str, usize>::new(),
            vect: Vector::<&'static str>::new(),
        }
    }

    pub fn len() -> usize {
        KEYWORDS.read().unwrap().vect.len()
    }

    pub fn get(key: &'static str) -> usize {
        let length = Keywords::len();
        let mut m = KEYWORDS.write().unwrap();
        
        match m.map.get(key) {
            // found entry
            Some(idx) => { *idx }
            None => {
                // add entry in vect and map
                m.vect.insert(length, key);
                m.map.insert(key, length);
                
                // return new index
                length
            }
        }
    }

    pub fn to_string(index: usize) -> &'static str {
        match KEYWORDS.read().unwrap().vect.get(index) {
            Some(key) => { *key }
            None => {""}
        }
    }

    pub fn init_static() {
        println!("init static");
    }
}

// Test with RwLock for now, change to 
lazy_static! {
    static ref KEYWORDS: RwLock<Keywords> = 
                        RwLock::new(Keywords::new());
}

#[test]
fn test_the_thing() {
    println!("Init state {:?}", Keywords::len());
    Keywords::init_static(); // expected to succeed
    println!("New state {:?}", Keywords::len());
    Keywords::get("essai");
    println!("add essai {:?} {}", Keywords::len(), 
                                    Keywords::to_string(0));
    Keywords::get("essai2");
    println!("add essai2 {:?} {} {}", Keywords::len(), 
                                    Keywords::to_string(0),
                                    Keywords::to_string(1));
    assert_eq!(1,1)
}
