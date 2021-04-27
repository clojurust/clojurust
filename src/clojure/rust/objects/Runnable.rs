use clojure::rust::*;

use crate::*;
// use clojure::lang::*;

pub trait Runnable: IObject+Callable+Thread {
    fn run(&self);
}
