//! Members definition

use crate::clojure::rust::function::*;

pub struct Member {
    name: usize,
    class: usize,
    getter: Option<SFunction>,
    setter: Option<SFunction>,
}

impl Member {
    pub fn new(
        name: usize,
        class: usize,
        getter: Option<SFunction>,
        setter: Option<SFunction>,
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
