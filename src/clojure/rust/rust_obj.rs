// use std::sync::*;
use lazy_static::lazy_static;
use im::hashmap::*;
use super::object::*;
use super::class::*;

pub struct RustObj {
    pub objects: HashMap<usize, Object>,
}

impl RustObj {
    fn new() -> RustObj {
        RustObj {
            objects: HashMap::<usize, Object>::new(),
        }
    }

    fn insert(name: usize, obj: Object) -> bool {
        RUST_OBJ.objects.insert(name, obj.clone());
        true
    }
}

pub fn init() {
    RUST_OBJ.objects.insert(9, Object::new::<Class>(0,&Class::new()));
}

lazy_static! {
//   static ref RUST_OBJ: Arc<RwLock<RustObj>> = Arc::new(RwLock::new(RustObj::new()));
    pub static ref RUST_OBJ: RustObj = RustObj::new();
}

