use crate::clojure;
// use clojure::rust::Object::*;
use clojure::lang::IPersistentCollection::*;
use clojure::lang::Sequable::*;
use clojure::lang::Sequential::*;
use clojure::rust::Counted::*;
use clojure::lang::ISeq::*;


pub trait IndexedSeq: Counted + IPersistentCollection + ISeq 
        + Sequable + Sequential {

}