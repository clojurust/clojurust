use std::error::Error;
use std::fmt;

use super::object;

pub struct SCljError {
    msg: String,
}

impl Error for SCljError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl fmt::Display for SCljError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Debug for SCljError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl object::TObject for SCljError {
    fn get_class(&self) -> &super::class::SClass {
        todo!()
    }

    fn call(&self, name: usize, args: &[object::Object]) -> object::Object {
        todo!()
    }

    fn get(&self, name: usize) -> object::Object {
        todo!()
    }

    fn to_string(&self) -> &str {
        todo!()
    }

    fn get_hash(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &object::Object) -> bool {
        todo!()
    }
}

pub fn error<T>(msg: &str) -> Result<&'static T, SCljError> {
    Err(SCljError {
        msg: String::from(msg),
    })
}
