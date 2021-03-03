use std::sync::Arc;

use im_rc;

use intertrait::cast::*;
use intertrait::*;

use lazy_static::__Deref;

use std::{any::*, fmt::*, hash::*, result::*, sync::*};

use super::class;
use super::object;

pub struct SPVector {
    inner: object::Object,
}

castable_to!(SPVector => [sync] object::TObject, PVector);

unsafe impl Send for SPVector {}

unsafe impl Sync for SPVector {}

pub trait PVector {
    fn len(&self) -> usize;

    fn get(&self, index: usize) -> object::Object;

    fn update(&self, index: usize, value: object::Object) -> SPVector;

    fn add(&mut self, value: object::Object) -> SPVector;
}

impl object::TObject for SPVector {
    fn get_class<'a>(&'a self) -> &'a class::SClass {
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

impl SPVector {
    pub fn new_vect(inner: im_rc::vector::Vector<object::Object>) -> SPVector {
        SPVector { inner }
    }

    pub fn new() -> SPVector {
        SPVector {
            inner: im_rc::vector::Vector::new(),
        }
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        object::Object::init();
    }
}

impl PVector for SPVector {
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn get(&self, index: usize) -> object::Object {
        match self.inner.get(index) {
            None => object::Object::null(),
            Some(o) => o.clone(),
        }
    }

    fn update(&self, index: usize, value: object::Object) -> SPVector {
        let &mut vec = &mut self.inner;
        SPVector {
            inner: vec.update(index, value.clone()),
        }
    }

    fn add(&mut self, value: object::Object) -> SPVector {
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
