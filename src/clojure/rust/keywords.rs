//! clojure::rust::keyword: keyword index

use std::cell::RefCell;
use im::vector::Vector;
use im::hashmap::HashMap;
use core::cell::RefMut;

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
        keywords.with(|kw| {
            kw.borrow().vect.len()
        })
    }
    
    pub fn get(key: &'static str) -> usize {
        let length = Keywords::len();
        keywords.with(|kw| {
            let mut m = kw.borrow_mut();
            
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
        })
    }

    pub fn to_string(index: usize) -> &'static str {
        keywords.with(|kw| {
            match kw.borrow().vect.get(index) {
                Some(key) => { *key }
                None => {""}
            }
        })
    }
    
    pub fn init_static() {
        println!("init static");
    }
}

// Test with thread-local for now, change to Atom afterward
thread_local!(
    #[allow(non_upper_case_globals)]
    static keywords: RefCell<Keywords> = 
                        RefCell::new(Keywords::new()));

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
