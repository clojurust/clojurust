// use std::sync::*;
use im_rc::vector::*;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

// use std::fmt::*;
use intertrait::cast::*;
use intertrait::*;

use super::object::*;
use super::pvector::*;

pub struct SRustObj {
    pub obj: Object,
}

castable_to!(Object => [sync] TObject);

pub trait RustObj {
    fn update(&self, index: usize, value: &Object) -> Object;

    fn add(&self, value: &Object) -> Object;

    fn get(&self, index: usize) -> Object;
}

impl RustObj for SRustObj {
    fn update(&self, index: usize, value: &Object) -> Object {
        let SRustObj { obj } = self;
        let inner = obj.clone().inner;
        match inner {
            None => todo!(),
            Some(o) => {
                let v = o.clone().as_ref().cast::<PVector>();
                match v {
                    Some(vect) => Object::new::<SRustObj>(SRustObj {
                        obj: {
                            if index >= vect.len() {
                                todo!();
                            } else {
                                Object::new::<SPVector>(vect.update(index, value.clone()))
                            }
                        },
                    }),
                    None => todo!(),
                }
            }
        }
    }

    fn add(&self, value: &Object) -> Object {
        let SRustObj { obj } = self;
        let inner = obj.clone().inner;
        match inner {
            None => todo!(),
            Some(o) => {
                let v = o.clone().as_ref().cast::<PVector>();
                match v {
                    Some(vect) => Object::new::<SRustObj>(SRustObj {
                        obj: { Object::new::<SPVector>(vect.add(value.clone())) },
                    }),
                    None => todo!(),
                }
            }
        }
    }

    fn get(&self, index: usize) -> Object {
        self.obj.get(index)
    }
}

impl SRustObj {
    pub fn new() -> Object {
        Object::new::<SRustObj>(SRustObj {
            obj: Object::new::<SPVector>(SPVector::new()),
        })
    }

    fn new_vect(new: Object) -> Object {
        Object::new::<SRustObj>(SRustObj { obj: new })
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        Object::init();
        SPVector::init();
    }
}

impl TObject for SRustObj {
    fn get_class(&self) -> &super::class::SClass {
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

static mut INIT: bool = false;

#[test]
fn test_rust_obj() {}
