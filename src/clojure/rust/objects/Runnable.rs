use crate::*;
use clojure::rust::*;
// use clojure::lang::*;

pub trait Runnable: IObject + Callable + Thread {
    fn run(&self);
}