//! clojure::rust::keyword: keyword index

use std::cell::RefCell;

use im::vector::Vector;
use im::hashmap::HashMap;

#[derive(Debug)]
struct KeywordsVect {
    data: Vector<&'static str>,
}

impl<'i> KeywordsVect {
    fn new() -> KeywordsVect {
        KeywordsVect {
            data: Vector::<&'static str>::new(),
        }
    }
}

#[derive(Debug)]
struct KeywordsMap {
    data: HashMap<&'static str, usize>
}

impl<'i> KeywordsMap {
    fn new() -> KeywordsMap {
        KeywordsMap {
            data: HashMap::<&'static str, usize>::new(),
        }
    }
}

struct Keywords {}

impl Keywords {
    pub fn len() -> usize {
        keywords_vect.with(|vect| 
            {
            vect.borrow().data.len()
        })
    }
    
    pub fn get(key: &'static str) -> usize {
        keywords_map.with(|map| {
            let mut m = map.borrow_mut();
            
            match m.data.get(key) {
                // found entry
                Some(idx) => { *idx }
                None => {
                    // add entry in vect and map
                    let length = Keywords::len();
                    
                    keywords_vect.with(|vect| {
                        vect.borrow_mut().data.insert(length, key);
                    });
                    
                    m.data.insert(key, length);
                    
                    // return new index
                    length
                }
            }
        })
    }

    pub fn to_string(index: usize) -> &'static str {
        keywords_vect.with(|vect| {
            match vect.borrow().data.get(index) {
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
    static keywords_vect: RefCell<KeywordsVect> = 
                        RefCell::new(KeywordsVect::new()));

thread_local!(
    #[allow(non_upper_case_globals)]
    static keywords_map: RefCell<KeywordsMap> = 
                        RefCell::new(KeywordsMap::new())
                    );

#[test]
fn test_the_thing() {
    println!("Init state {:?}", Keywords::len());
    Keywords::init_static(); // expected to succeed
    println!("New state {:?}", Keywords::len());
    Keywords::get("essai");
    println!("add essai {:?} {}", Keywords::len(), Keywords::to_string(0));
    assert_eq!(1,1)
}
