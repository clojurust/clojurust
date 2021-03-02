use std::sync::Arc;

pub use im_rc::vector::*;
use im_rc::*;
use intertrait::cast::*;
use intertrait::*;
use lazy_static::__Deref;
use std::{any::*, fmt::*, hash::*, result::*, sync::*};

use super::class::*;
use super::object::*;

pub struct SPVector {
    inner: Vector<Object>,
}

castable_to!(SPVector => [sync] TObject, PVector);

unsafe impl Send for SPVector {}

unsafe impl Sync for SPVector {}

pub trait PVector {
    fn len(&self) -> usize;

    fn get(&self, index: usize) -> Object;

    fn update(&self, index: usize, value: Object) -> SPVector;

    fn add(&mut self, value: Object) -> SPVector;
}

impl TObject for SPVector {
    fn get_class<'a>(&'a self) -> &'a SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[Object]) -> Object {
        todo!()
    }

    fn get(&self, name: usize) -> Object {
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

impl SPVector {
    pub fn new_vect(inner: Vector<Object>) -> SPVector {
        SPVector { inner }
    }

    pub fn new() -> SPVector {
        SPVector {
            inner: Vector::new(),
        }
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

impl PVector for SPVector {
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn get(&self, index: usize) -> Object {
        match self.inner.get(index) {
            None => Object::null(),
            Some(o) => o.clone(),
        }
    }

    fn update(&self, index: usize, value: Object) -> SPVector {
        let &mut vec = &mut self.inner;
        SPVector {
            inner: vec.update(index, value.clone()),
        }
    }

    fn add(&mut self, value: Object) -> SPVector {
        let mut vect = &self.inner;
        vect.push_back(value.clone());
        SPVector { inner: *vect }
    }
}

// impl Clone for PVector {
//     fn clone(&self) -> Self {
//         PVector {
//             inner: self.inner.clone(),
//         }
//     }
// }

impl std::fmt::Debug for SPVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self.cast::<SPVector>();
        match a {
            Some(spv) => f.write_str(spv.to_string()),
            None => todo!(),
        }
    }
}

impl Hash for SPVector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.get_hash())
    }
}

static mut INIT: bool = false;
