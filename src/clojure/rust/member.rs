//! Members definition

use super::function::*;

pub struct Member {
    name: usize,
    class: usize,
    getter: Option<Function>,
    setter: Option<Function>,
}

impl Member {
    pub fn new(name: usize, 
               class: usize, 
               getter: Option<Function>, 
               setter: Option<Function>) -> Member {
        Member {
            name,
            class,
            getter,
            setter,
        }

    }    
}

pub fn init() {

}