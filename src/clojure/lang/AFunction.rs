// use intertrait::cast::*;
use crate::*;

use_obj! {
    clojure::rust::Object;
    clojure::rust::Class;
    clojure::rust::Serializable;
    clojure::rust::Comparable;
    clojure::rust::Reversible;
    clojure::rust::Counted;
    clojure::rust::Iterable;
    clojure::rust::Indexed;
    clojure::rust::Associative;
    clojure::rust::ObjError;
    clojure::rust::RandomAccess;
    clojure::rust::List;
    clojure::lang::APersistentVector;
    clojure::lang::IPersistentVector;
    clojure::lang::IPersistentStack;
    clojure::lang::IPersistentCollection;
    clojure::lang::ITransientCollection;
    clojure::lang::IObj;
    clojure::lang::IMeta;
    clojure::lang::IHashEq;
    clojure::lang::Sequable;
    clojure::lang::IKVReduce;
    clojure::lang::IEditableCollection;
}

init_obj! {
    PersistentVector {
        clojure::rust::Object::init();
        clojure::rust::Class::init();
    }
}
