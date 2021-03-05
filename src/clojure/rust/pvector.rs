// use lazy_static::__Deref;
// use std::{any::*, result::*, sync::*};
// use std::{fmt::*, hash::*};

// use intertrait::cast::*;
use intertrait::*;

use super::class::*;
use super::object::*;

pub type ObjVector = im::vector::Vector<Object>;

pub struct SPVector {
    inner: ObjVector,
}

castable_to!(SPVector => [sync] TObject, PVector);

castable_to!(ObjVector => [sync] TObject);

unsafe impl Send for SPVector {}

unsafe impl Sync for SPVector {}

impl TObject for ObjVector {
    fn get_class(&self) -> &SClass {
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
    pub fn new_vect(inner: ObjVector) -> SPVector {
        SPVector { inner }
    }

    pub fn new() -> SPVector {
        SPVector {
            inner: ObjVector::new(),
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

/// #[deprecated(since = "0.1.0", note = "Standard traits are only defined in the Object struct")]
// impl Clone for PVector {
//     fn clone(&self) -> Self {
//         PVector {
//             inner: self.inner.clone(),
//         }
//     }
// }

/// #[deprecated(since = "0.1.0", note = "Standard traits are only defined in the Object struct")]
// impl Debug for SPVector {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let a = self.cast::<SPVector>();
//         match a {
//             Some(spv) => f.write_str(spv.to_string()),
//             None => panic!("Optional None should not apply"),
//         }
//     }
// }

/// #[deprecated(since = "0.1.0", note = "Standard traits are only defined in the Object struct")]
// impl Hash for SPVector {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         if let Some(o_vec) = self.cast::<TObject>() {
//             state.write_usize(o_vec.get_hash())
//         } else {
//             panic!("Optional None should not apply");
//         }
//     }
// }

static mut INIT: bool = false;
