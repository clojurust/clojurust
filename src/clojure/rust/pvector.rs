use std::sync::Arc;

pub use im::vector::*;
use im::*;
use intertrait::cast::*;
use intertrait::*;
use lazy_static::__Deref;
use std::{any::*, fmt::*, hash::*, result::*, sync::*};

use super::class::*;
use super::object::*;

pub struct PVector {
    inner: Vector<Object>,
}

castable_to!(PVector => [sync] TObject);

impl TObject for PVector {
    fn get_class<'a>(&'a self) -> &'a SClass {
        todo!()
    }

    fn call(&self, name: &str, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: &str) -> Object {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }
}

impl PVector {
    pub fn new_vect(inner: Vector<Object>) -> PVector {
        PVector { inner }
    }

    pub fn new() -> PVector {
        PVector {
            inner: Vector::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, index: usize) -> Object {
        match self.inner.get(index) {
            None => Object::null().clone(),
            Some(o) => o.clone(),
        }
    }

    pub fn update(&self, index: usize, value: &Object) -> PVector {
        let &mut vec = &mut self.inner;
        PVector {
            inner: vec.update(index, value.clone()),
        }
    }

    pub fn add(&mut self, value: &Object) -> PVector {
        let mut vect = &self.inner;
        vect.push_back(value.clone());
        PVector { inner: *vect }
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        Object::init();
    }
}

// impl Clone for PVector {
//     fn clone(&self) -> Self {
//         PVector {
//             inner: self.inner.clone(),
//         }
//     }
// }

// impl std::fmt::Debug for PVector {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let a = self;
//         f.write_str(&self.to_string())
//     }
// }

// impl Hash for PVector {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         state.write_usize(self.get_hash())
//     }

//     fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
//     where
//         Self: Sized,
//     {
//         for piece in data {
//             piece.hash(state);
//         }
//     }
// }

static mut INIT: bool = false;
