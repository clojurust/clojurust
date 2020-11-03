//! Object
//!
//! Defines the Java Object and Class

use query_interface::Object as RustObject;
use query_interface::{ObjectClone, HasInterface};
use rc::Rc;

pub trait CljObject: RustObject + ObjectClone + 
                        HasInterface<dyn ObjectClone> { }
mopo!(dyn CljObject);

#[allow(dead_code)]
pub struct Object {
    inner: Rc<&'static dyn CljObject>,
}

#[allow(dead_code)]
impl Object {
    pub fn new(obj: &'static dyn CljObject) -> Object {
        Object {
            inner: Rc::new(obj),
        }
    }
}
