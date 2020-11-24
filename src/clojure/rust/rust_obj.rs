// use std::sync::*;
use lazy_static::lazy_static;
use im::hashmap::*;
use super::object::*;

struct RustObj {
    pub objects: HashMap<usize, Object>,
}

impl RustObj {
    fn new() -> RustObj {
        RustObj {
            objects: HashMap::<usize, Object>::new(),
        }
    }

    fn add(name: &str, obj: Object) -> usize {
        0
    }
}

lazy_static! {
//   static ref RUST_OBJ: Arc<RwLock<RustObj>> = Arc::new(RwLock::new(RustObj::new()));
    static ref RUST_OBJ: RustObj = RustObj::new();
}

