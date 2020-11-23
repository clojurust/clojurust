use std::{sync::Arc, mem::transmute};
use lazy_static::lazy_static;

struct RustObj {
    classes: Arc<RwLock<Classes>>,
}

impl RustObj {
    fn new() -> RustObj {
        RustObj {
            classes: Arc::new(RwLock::new(Classes::new())),
        }
    }
}

lazy_static! {
    RUST_OBJ: RustObj = RustObj::new();
}
