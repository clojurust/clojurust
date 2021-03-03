use lazy_static::lazy_static;

use std::sync::{Arc, RwLock};

use intertrait::cast::*;
use intertrait::*;

use super::object;
use super::pvector;

pub struct SRustObj {
    pub obj: object::Object,
}

castable_to!(SRustObj => [sync] object::TObject, RustObj);

pub trait RustObj {
    fn update(&self, index: usize, value: &object::Object) -> object::Object;

    fn add(&self, value: &object::Object) -> object::Object;

    fn get(&self, index: usize) -> object::Object;
}

impl RustObj for SRustObj {
    fn update(&self, index: usize, value: &object::Object) -> object::Object {
        let SRustObj { obj } = self;
        let inner = obj.clone().inner;
        match inner {
            None => todo!(),
            Some(o) => {
                let v = o.clone().as_ref().cast::<pvector::PVector>();
                match v {
                    Some(vect) => object::Object::new::<SRustObj>(SRustObj {
                        obj: {
                            if index >= vect.len() {
                                todo!();
                            } else {
                                object::Object::new::<pvector::SPVector>(
                                    vect.update(index, value.clone()),
                                )
                            }
                        },
                    }),
                    None => todo!(),
                }
            }
        }
    }

    fn add(&self, value: &object::Object) -> object::Object {
        let SRustObj { obj } = self;
        let inner = obj.clone().inner;
        match inner {
            None => todo!(),
            Some(o) => {
                let v = o.clone().as_ref().cast::<pvector::PVector>();
                match v {
                    Some(vect) => object::Object::new::<SRustObj>(SRustObj {
                        obj: { object::Object::new::<pvector::SPVector>(vect.add(value.clone())) },
                    }),
                    None => todo!(),
                }
            }
        }
    }

    fn get(&self, index: usize) -> object::Object {
        self.obj.get(index)
    }
}

impl SRustObj {
    pub fn new() -> SRustObj {
        SRustObj {
            obj: object::Object::new::<pvector::SPVector>(pvector::SPVector::new()),
        }
    }

    fn new_vect(new: object::Object) -> object::Object {
        object::Object::new::<SRustObj>(SRustObj { obj: new })
    }

    pub unsafe fn init() {
        // only execute one time
        if INIT {
            return;
        }
        INIT = true;

        // Insures all is initialized
        object::Object::init();
        pvector::SPVector::init();
    }
}

impl object::TObject for SRustObj {
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

static mut INIT: bool = false;

#[test]
fn test_rust_obj() {}
