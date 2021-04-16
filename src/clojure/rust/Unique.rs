//! `Unique` association of names with unique id
//!
//!

use std::{fmt::*, sync::*};

use cast::CastArc;
use lazy_static::lazy_static;

use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::ObjError;
    clojure::rust::Class;
    clojure::rust::Counted;
    clojure::lang::PersistentMap;
    clojure::lang::PersistentVector;
}

// castable_to!(SUnique => [sync] TObject, Unique);

init_obj! {
    SUnique {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
        clojure::lang::PersistentMap::init();
        clojure::lang::PersistentVector::init();
        }
}

/// # A keyword storage structure
///
/// We will store all `String`s used as reference to objects as `usize`.
/// * `usize` values are unique and immutable for every `String`.
/// * `Strings` are added incrementally to the `vect` `ObjVector` and cannot
/// be destroyed.
///
/// As a `String` is added, it's index is added in the `map` `ObjHashMap`.
///
/// # Examples
#[derive(Debug)]
pub struct SUnique {
    /// `SPersistentMap` of `name`: `String` -> `id`: `usize`
    pub map: Object,

    /// `SPersistentVector` `index`: `usize` -> `value`: `String`
    pub vect: Object,
}

unsafe impl Send for SUnique {}

unsafe impl Sync for SUnique {}

/// Protocole `Unique`
pub trait Unique: CastFromSync {
    /// Size of SStrVector
    fn len(&self) -> usize;

    /// Gives name of index
    ///
    /// return None if doesn't exist
    fn get_name<'a>(&self, index: usize) -> ObjResult<String>;

    /// Gives index of name
    ///
    /// Create name and index is they doesn't exist
    fn get_or_make_index(&mut self, index: &str) -> ObjResult<usize>;

    /// Gives index of name
    ///
    /// return None if doesn't exist
    fn get_index(&mut self, name: &str) -> ObjResult<usize>;

    /// Tests if name exists
    fn test(&self, key: &str) -> ObjResult<bool>;
}

use crate::new_obj;

// `Implementation` of `Protocol` `Unique` for `SUnique`
impl Unique for SUnique {
    /// Size of SStrVector
    fn len(&self) -> usize {
        match self.vect.inner {
            Some(v) => {
                match v.cast::<Counted>() {
                    Ok(i) => {
                        match i.as_ref().count() {
                            Ok(s) => s,
                            _ => panic!("Severe: SUnique non initialized"),
                        }
                    },
                    _ => panic!("Severe: SUnique non initialized"),
                }
            },
            _ => panic!("Severe: SUnique non initialized"),
        }
    }

    /// Gives name of index
    ///
    /// return None if doesn't exist
    fn get_name(&self, key: usize) -> ObjResult<String> {
        if let Some(v) = self.vect.cast::<SPersistentVector>() {
            return v.get(key);
        }
        panic!("Severe: SUnique non initialized");
    }

    /// Gives index of name
    ///
    /// Create name and index is they doesn't exist
    fn get_or_make_index(&mut self, name: &str) -> ObjResult<usize> {
        if let Some(m) = self.map.cast_mut::<SPersistentMap>() {
            if let Some(v) = self.vect.cast_mut::<SPersistentVector>() {
                if let Some(o) = m.get(name) {
                    return *o;
                } else {
                    let length = self.len();
                    v.push_back(String::from(name));
                    *m = m.update(String::from(name), length);

                    let k = SUnique {
                        map: new_obj!(*m),
                        vect: new_obj!(*v),
                    };
                    *self = k;

                    // return new index that was the length of the vector
                    return length;
                }
            }
        }
        panic!("Severe: SUnique non initialized");
    }

    /// Gives index of name
    ///
    /// return None if doesn't exist
    fn get_index(&mut self, name: &str) -> ObjResult<usize> {
        if let Some(m) = self.map.cast_mut::<SPersistentMap>() {
            if let Some(v) = self.vect.cast_mut::<SPersistentVector>() {
                if let Some(o) = m.get(name) {
                    return Ok(*o);
                } else {
                    return None;
                }
            }
        }
        panic!("Severe: SUnique non initialized");
    }

    /// Tests if name exists
    fn test(&self, name: &str) -> ObjResult<bool> {
        if let Some(m) = self.map.cast::<SPersistentMap>() {
            match m.get(name) {
                Some(_) => return true,
                None => return false,
            }
        }
        panic!("Severe: SUnique non initialized");
    }
}

impl TObject for SUnique {
    fn get_class<'a>(&self) -> &'a SClass {
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
            map: new_obj!(SPersistentMap::default()),
            vect: new_obj!(SPersistentVector::default()),
        }
    }
}

impl Display for SUnique {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SUnique")
    }
}

impl SUnique {
    pub fn new() -> Object {
        new_obj!(SUnique::default())
    }
}

impl Drop for SUnique {
    fn drop(&mut self) {
        println!("Dropping Keyword state! -> {:?}", self.to_string());
    }
}

lazy_static! {
    /// Methods short names
    static ref METHODS: Object = Object { inner: None };
    /// Members short names
    static ref MEMBERS: Object = Object { inner: None };
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
