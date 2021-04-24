use clojure::lang::*;
use clojure::rust::*;

use crate::*;

pub trait ASeq: IObject+ISeq+Sequential+List+Serializable+IHashEq
{
}
