//! Anonymous Function with one defined arity, and possible last element multi arity

use super::object::*;
use std::sync::*;

#[derive(Clone)]
pub struct Implementation {
    pub multiary: bool,
    pub function: &'static Fn(&[Arc<Object>]) -> Arc<Object>,
}

impl Implementation {
    pub fn new(
        multiary: bool,
        function: &'static Fn(&[Arc<Object>]) -> Arc<Object>,
    ) -> Implementation {
        Implementation { multiary, function }
    }

    pub fn call(&self, args: &[Arc<Object>]) -> Arc<Object> {
        self.function.call((args,))
    }
}

pub fn init() {}
