//! Unique name associated with unique id
//!
//!

use std::{borrow::BorrowMut, sync::*};

// use lazy_static::{__Deref};
use intertrait::cast::*;

use crate::use_obj;

use_obj! {
    clojure::rust::number;
    clojure::rust::object;
    clojure::rust::str_hashmap;
    clojure::rust::str_vector;
    clojure::rust::stri;
    clojure::rust::class;
}

castable_to!(SUnique => [sync] TObject, Unique);

init_obj! {
    Stri {
        clojure::rust::obj_vector::init();
        clojure::rust::object::init();
        clojure::rust::str_hashmap::init();
        clojure::rust::str_vector::init();
        clojure::rust::stri::init();
        clojure::rust::class::init();
    }
}

/// # A keyword storage structure
///
/// We will store all `String`s used as reference to objects as `usize`.
/// ** `usize` values are unique and immutable for every `String`.
/// ** `Strings` are added incrementally to the `vect` `StrVector` and cannot
/// be destroyed.
///
/// As a `String` is added, it's index is added in the `map` `StrHashMap`.
///
/// # Examples

pub struct SUnique {
    pub map: Object,  // SStrHashMap,
    pub vect: Object, // SStrVector,
}

castable_to!(SUnique => [sync] TObject, Unique);

unsafe impl Send for SUnique {}

unsafe impl Sync for SUnique {}

pub trait Unique: CastFromSync {}

impl Unique for SUnique {}

impl TObject for SUnique {
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

impl Default for SUnique {
    fn default() -> Self {
        SUnique {
            map: Object::new(Arc::new(SStrHashMap::default())),
            vect: Object::new(Arc::new(SStrVector::default())),
        }
    }
}

impl SUnique {
    pub fn new() -> Object {
        Object::new(Arc::new(SUnique::default()))
    }

    pub fn get_mut<'a>(&'a self) -> &'a mut SUnique {
        self.borrow_mut()
    }

    pub fn len(&self) -> usize {
        self.vect.inn::<SStrVector>().len()
    }

    pub fn get_path_o(&self, key: Object) -> Object {
        let k = key.inner.as_ref();
        let ko = k.cast::<Usize>();
        let v = self.vect.inn::<SStrVector>();
        v.get(ko).unwrap().clone()
    }

    pub fn get_path<'a>(&self, key: usize) -> String {
        let v = self.vect.inn::<SStrVector>();
        let s = v.get(key).unwrap();
        s.inn::<SStri>().inner
    }

    pub fn get_maybe_key(&mut self, key: &str) -> usize {
        let m = self.map.clone().inn_mut::<SStrHashMap>();
        let v = self.vect.clone().inn_mut::<SStrVector>();
        let length = self.len();

        match m.get(key) {
            // found entry
            Some(idx) => *idx,

            // Not found: add entry in vect and map
            None => {
                v.push_back(String::from(key));
                *m = m.update(String::from(key), length);

                let k = SUnique { map: m, vect: v };
                *self = k;

                // return new index that was the length of the vector
                length
            }
        }
    }

    pub fn test(key: String, keywords: &SUnique) -> bool {
        let i = SUnique::get(keywords).clone();
        let a = i;
        match a.map.get(&key) {
            Some(_) => true,
            None => false,
        }
    }
}

impl Drop for SUnique {
    fn drop(&mut self) {
        println!("Dropping Keyword state! -> {:?}", self.to_string());
    }
}

pub fn init_keywords() -> RwLock<Object> {
    RwLock::new(SUnique::new())
}

#[test]
fn test_keywords() {
    // // Initial state
    // println!(
    //     "Init state len = {:?} state = {:?}",
    //     SUnique::len(&CORE),
    //     SUnique::get(&CORE)
    // );

    // let e1 = SUnique::get(&CORE);

    // // Call init_static
    // println!(
    //     "New state len = {:?} state = {:?}",
    //     SUnique::len(&CORE),
    //     SUnique::get(&CORE)
    // );

    // let e2 = SUnique::get(&CORE);

    // // add first keyword
    // let o = SUnique::get_key(&String::from("essai"), &CORE);
    // println!(
    //     "add essai len = {:?} state = {:?}",
    //     SUnique::len(&CORE),
    //     SUnique::get(&CORE)
    // );

    // let e3 = SUnique::get(&CORE);

    // // add second keyword
    // SUnique::get_key(&"essai2".to_string(), &CORE);
    // println!(
    //     "add essai2 len = {:?} state = {:?}",
    //     SUnique::len(&CORE),
    //     SUnique::get(&CORE)
    // );

    // let e4 = SUnique::get(&CORE);

    // // display existing keywords
    // println!("Keyword 0 = \"{}\"", SUnique::get_id(0, &CORE));
    // println!("Keyword 1 = \"{}\"", SUnique::get_id(1, &CORE));
    // println!("Keyword 2 = \"{}\"", SUnique::get_id(2, &CORE));

    // // Verify persistant state
    // println!("State 1 = {:?}", e1);
    // println!("State 2 = {:?}", e2);
    // println!("State 3 = {:?}", e3);
    // println!("State 4 = {:?}", e4);

    // assert_eq!(1, 1)

    // // display of droppings
    // // At the output state 1 and 2 are the same and so only one drop,
    // // event is there are two Arc, but it's a lone struct.
    // // State 3 is droped as it's the output of test function, and
    // // e3 is the only link to the state.
    // // As state 4 is linked in the KEYWORDS global variable, the drop
    // // is not displayed..
}
