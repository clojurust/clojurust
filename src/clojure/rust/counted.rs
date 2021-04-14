/// Protocol `Counted`

use crate::init_obj;
use crate::init_init_obj;

// castable_to!(SGlobals => [sync] TObject, Globals);

init_obj! {
    Counted {
    }
}

pub trait Counted {
    /// give the nr of elements
    fn count(&self) -> usize;
}