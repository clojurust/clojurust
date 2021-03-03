//! clojure::rust::keyword: keyword index

use im_rc::hashmap::HashMap;
use im_rc::*;
use lazy_static::{__Deref, lazy_static};
use std::sync::*;

use intertrait::cast::*;
use intertrait::*;

use super::object;

/// A keyword storage structure
///
/// We will store all `String`s used as reference to objects as `usize`.
/// `usize` values are unique and immutable for every `String`.
/// `Strings` are added incrementally to the `vect` `Vector` and cannot be destroyed.
/// as a `String` is added, it's index is added in the `map` `HashMap`.
/// # Examples

#[derive(Default, Debug)]
pub struct SUnique {
    map: HashMap<String, usize>,
    vect: Vector<String>,
}

castable_to!(SUnique => [sync] object::TObject, Unique);

unsafe impl Send for SUnique {}

unsafe impl Sync for SUnique {}

pub trait Unique {}

impl Unique for SUnique {}

impl object::TObject for SUnique {
    fn get_class(&self) -> &super::class::SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[object::Object]) -> object::Object {
        todo!()
    }

    fn get(&self, name: usize) -> object::Object {
        todo!()
    }

    fn to_string(&self) -> &str {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &object::Object) -> bool {
        todo!()
    }
}

impl SUnique {
    pub fn new() -> SUnique {
        SUnique {
            map: HashMap::<String, usize>::new(),
            vect: Vector::<String>::new(),
        }
    }

    pub fn current(self: &RwLock<Arc<SUnique>>) -> Arc<SUnique> {
        self.read().unwrap().clone()
    }

    pub fn current_mut(keywords: &RwLock<Arc<SUnique>>) -> Arc<SUnique> {
        keywords.write().unwrap().clone()
    }

    pub fn make_current(self, keywords: &RwLock<Arc<SUnique>>) {
        *keywords.write().unwrap() = Arc::new(self);
    }

    pub fn len(keywords: &RwLock<Arc<SUnique>>) -> usize {
        SUnique::current(keywords).vect.len()
    }

    pub fn get_id(key: usize, keywords: &RwLock<Arc<SUnique>>) -> String {
        let v = &SUnique::current(keywords).vect;
        if v.len() < key + 1 {
            String::from("...vide...")
        } else {
            v.get(key).unwrap().clone()
        }
    }

    pub fn get_key(key: &str, keywords: &RwLock<Arc<SUnique>>) -> usize {
        let i = SUnique::current(keywords).clone();
        let a = i.as_ref();
        let mut m = a.map.clone();
        let mut v = a.vect.clone();
        let length = SUnique::len(keywords);

        let k = key.to_string();
        match m.get(&k) {
            // found entry
            Some(idx) => *idx,

            // Not found: add entry in vect and map
            None => {
                // Insert new values in vector and map
                v.push_back(k.clone());
                m = m.update(k.clone(), length);

                let k = SUnique { map: m, vect: v };
                k.make_current(keywords);

                // return new index that was length of vector
                length
            }
        }
    }

    pub fn test(key: String, keywords: &RwLock<Arc<SUnique>>) -> bool {
        let i = SUnique::current(keywords).clone();
        let a = i.as_ref();
        match a.map.get(&key) {
            Some(_) => true,
            None => false,
        }
    }

    pub unsafe fn init() {}
}

impl Drop for SUnique {
    fn drop(&mut self) {
        println!("Dropping Keyword state! -> {:?}", self);
    }
}

static INIT: bool = false;

pub fn init_keywords() -> RwLock<Arc<SUnique>> {
    RwLock::new(Arc::new(Default::default()))
}

lazy_static! {
    /// Private access to static `Keywords` struture.
    ///
    /// Here will be stored and retrived keywords data.
    pub static ref KEYWORDS: RwLock<Arc<SUnique>> = init_keywords();
    pub static ref CORE: RwLock<Arc<SUnique>> = init_keywords();
}

#[test]
fn test_keywords() {
    // Initial state
    println!(
        "Init state len = {:?} state = {:?}",
        SUnique::len(&CORE),
        SUnique::current(&CORE)
    );

    let e1 = SUnique::current(&CORE);

    // Call init_static
    println!(
        "New state len = {:?} state = {:?}",
        SUnique::len(&CORE),
        SUnique::current(&CORE)
    );

    let e2 = SUnique::current(&CORE);

    // add first keyword
    let o = SUnique::get_key(&String::from("essai"), &CORE);
    println!(
        "add essai len = {:?} state = {:?}",
        SUnique::len(&CORE),
        SUnique::current(&CORE)
    );

    let e3 = SUnique::current(&CORE);

    // add second keyword
    SUnique::get_key(&"essai2".to_string(), &CORE);
    println!(
        "add essai2 len = {:?} state = {:?}",
        SUnique::len(&CORE),
        SUnique::current(&CORE)
    );

    let e4 = SUnique::current(&CORE);

    // display existing keywords
    println!("Keyword 0 = \"{}\"", SUnique::get_id(0, &CORE));
    println!("Keyword 1 = \"{}\"", SUnique::get_id(1, &CORE));
    println!("Keyword 2 = \"{}\"", SUnique::get_id(2, &CORE));

    // Verify persistant state
    println!("State 1 = {:?}", e1);
    println!("State 2 = {:?}", e2);
    println!("State 3 = {:?}", e3);
    println!("State 4 = {:?}", e4);

    assert_eq!(1, 1)

    // display of droppings
    // At the output state 1 and 2 are the same and so only one drop,
    // event is there are two Arc, but it's a lone struct.
    // State 3 is droped as it's the output of test function, and
    // e3 is the only link to the state.
    // As state 4 is linked in the KEYWORDS global variable, the drop
    // is not displayed..
}
