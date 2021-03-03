//! Members definition

use crate::clojure::lang::function;

pub struct Member {
    name: usize,
    class: usize,
    getter: Option<function::Function>,
    setter: Option<function::Function>,
}

impl Member {
    pub fn new(
        name: usize,
        class: usize,
        getter: Option<function::Function>,
        setter: Option<function::Function>,
    ) -> Member {
        Member {
            name,
            class,
            getter,
            setter,
        }
    }
}

pub fn init() {}
