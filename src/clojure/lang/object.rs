/// IObject trait
/// 
/// Defines the Java Object Interface
pub trait IObject {
    type Type;
}

enum Object {
    Ref(Rc<dyn IObject>),
    Null,
}

struct I32 {
    Value: i32,
}

impl IObject for I32 { 
    type Type = I32;
}

impl I32 {
    fn new(value: i32) -> I32 {
        I32 {Value: value}
    }
}

fn makeI32(value: i32) -> Rc<dyn IObject> {
    Rc::new(<I32 as IObject>::Type::new(value))
}

