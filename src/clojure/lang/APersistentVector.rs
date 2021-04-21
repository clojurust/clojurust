//! # Vector of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` Vector<Object> library

use crate::*;

use_obj! {
    // APersistentVector
    clojure::rust::Object; //
    clojure::rust::IObject; //
    clojure::rust::ObjResult;
    clojure::rust::List;
    clojure::rust::RandomAccess;
    clojure::rust::Comparable;
    clojure::rust::Iterable;
    clojure::rust::Counted;
    clojure::rust::Indexed;
    clojure::rust::Serializable;
    clojure::lang::IPersistentCollection;
    clojure::lang::IPersistentStack;
    clojure::lang::IPersistentVector;
    clojure::lang::APersistentVector;
    clojure::lang::IHashEq;
    clojure::lang::Sequable;
    clojure::lang::Sequential;
    clojure::rust::Reversible;
    clojure::rust::Collection;

    // RSeq
    clojure::lang::IndexedSeq;
    clojure::rust::Counted;
    clojure::lang::ASeq;
    clojure::lang::ISeq;

    // SubVector
    clojure::lang::IObj;
    clojure::lang::IMeta;
    clojure::rust::Associative;
    
}

use intertrait::*;
castable_to!(SRSeq => [sync] IObject, ASeq,  
    IndexedSeq, Counted);

castable_to!(SSubVector => [sync] IObject, IObj,  
        APersistentVector);

init_obj! {
    APersistentVector {
        // APersistentVector
        clojure::rust::Object::init();
        clojure::lang::IPersistentVector::init();
        clojure::rust::List::init();
        clojure::rust::RandomAccess::init();
        clojure::rust::Comparable::init();
        clojure::rust::Iterable::init();
        clojure::rust::Serializable::init();
        clojure::lang::IHashEq::init();

        // RSeq
        clojure::lang::IndexedSeq::init();
        clojure::rust::Counted::init();
        clojure::lang::ASeq::init();

        // SubVector
        clojure::lang::IObj::init();
        clojure::lang::IMeta::init();
    }
}

pub trait APersistentVector: IObject + IPersistentVector + Iterable 
                        + List + RandomAccess + Comparable + Serializable
                        + IHashEq 
{
    fn _hash(&self) -> usize;
    fn _hash_eq(&self) -> usize;
}

pub struct SRSeq {
    v: Object, //&'a IPersistantVector,
    i: usize,
}

pub trait RSeq: IObject + ASeq + IndexedSeq + Counted {

}

impl IObject for SRSeq {
    fn getClass<'a>(&self) -> &'a clojure::rust::Class::SClass {
        todo!()
    }

    fn hashCode(&self) -> usize {
        todo!()
    }

    fn toString(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }
}

impl IObj for SRSeq {
    fn withMeta(&self, meta: &Object) -> ObjResult<Object> {
        todo!()
    }
}

impl ASeq for SRSeq {
}

impl IMeta for SRSeq {
    fn meta(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IHashEq for SRSeq {
    fn hasheq(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl Serializable for SRSeq {
}

impl Sequential for SRSeq {
}

impl Counted for SRSeq {
    fn count(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl IndexedSeq for SRSeq {
    fn index(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl Sequable for SRSeq {
    fn seq(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl Iterable for SRSeq {
    fn iterator(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl ISeq for SRSeq {
    fn cons(&self, o: &Object) -> ObjResult<Object> {
        todo!()
    }
}

impl List for SRSeq {
}

impl IPersistentCollection for SRSeq {
    fn cons(&self, o: &Object) -> ObjResult<Object> {
        todo!()
    }

    fn count(&self) -> ObjResult<usize> {
        todo!()
    }

    fn empty(&self) -> ObjResult<Object> {
        todo!()
    }

    fn equiv(&self, o: Object) -> ObjResult<bool> {
        todo!()
    }
}

impl Collection for SRSeq {
        fn size(&self) -> usize {
        todo!()
    }

    fn isEmpty(&self) -> bool {
        todo!()
    }

    fn contains(&self) -> ObjResult<bool> {
        todo!()
    }

    fn toArray(&self) -> ObjResult<Vec<Object>> {
        todo!()
    }

    fn containsAll(&self, c: &Object) -> ObjResult<bool> {
        todo!()
    }
}

pub struct SSubVector {
    v: Object, // &'a IPersistantVector,
    start: usize,
    end: usize,
    _meta: Object, //&'a IPersistantMap,
}

pub trait SubVector: IObject + IObj + APersistentVector {
    fn v(&self) -> ObjResult<Object>;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
}

impl SubVector for SSubVector {
    fn v(&self) -> ObjResult<Object> {
        Ok(self.v.clone())
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}

impl IObj for SSubVector {
    /// SSubVector -> IObj
    fn withMeta(&self, meta: &Object) -> ObjResult<Object> {
        todo!()
    }
}

impl IPersistentVector for SSubVector {
    fn assocN(&self, i: usize, val: &Object) -> ObjResult<Object> {
        todo!()
    }

    fn cons(&self, o: Object) -> ObjResult<Object> {
        todo!()
    }

    fn length(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl IPersistentStack for SSubVector {
    fn peek(&self) -> ObjResult<Object> {
        todo!()
    }

    fn pop(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl IPersistentCollection for SSubVector {
    fn cons(&self, o: &Object) -> ObjResult<Object> {
        todo!()
    }

    fn count(&self) -> ObjResult<usize> {
        todo!()
    }

    fn empty(&self) -> ObjResult<Object> {
        todo!()
    }

    fn equiv(&self, o: Object) -> ObjResult<bool> {
        todo!()
    }
}

impl APersistentVector for SSubVector {
    fn _hash(&self) -> usize {
        todo!()
    }

    fn _hash_eq(&self) -> usize {
        todo!()
    }
}

impl IHashEq for SSubVector {
    fn hasheq(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl Iterable for SSubVector {
    fn iterator(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl Reversible for SSubVector {
    fn rseq(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl Sequable for SSubVector {
    fn seq(&self) -> ObjResult<Object> {
        todo!()
    }
}

impl Counted for SSubVector {
    fn count(&self) -> ObjResult<usize> {
        todo!()
    }
}

impl Indexed for SSubVector {
    fn nth_1(&self, i: usize) -> ObjResult<Object> {
        todo!()
    }

    fn nth_2(&self, i: usize, notFound: Object) -> ObjResult<Object> {
        todo!()
    }
}

impl List for SSubVector {
    
}

impl Collection for SSubVector {
    fn size(&self) -> usize {
        todo!()
    }

    fn isEmpty(&self) -> bool {
        todo!()
    }

    fn contains(&self) -> ObjResult<bool> {
        todo!()
    }

    fn toArray(&self) -> ObjResult<Vec<Object>> {
        todo!()
    }

    fn containsAll(&self, c: &Object) -> ObjResult<bool> {
        todo!()
    }
}

impl Comparable for SSubVector {
    fn compareTo(&self, o: &Object) -> ObjResult<i8> {
        todo!()
    }
}

impl RandomAccess for SSubVector {
    
}

impl Serializable for SSubVector {
    
}

impl Associative for SSubVector {
    fn assoc(&self, key: &Object, value: &Object) -> ObjResult<Object> {
        todo!()
    }

    fn containsKey(&self, key: &Object) -> ObjResult<bool> {
        todo!()
    }

    fn entryAt(&self, key: &Object) -> ObjResult<Object> {
        todo!()
    }
}

impl IMeta for SSubVector {
    /// SSubVector -> IPersistentMap
    fn meta(&self) -> ObjResult<Object> {
        Ok(self._meta.clone())
    }
}

impl IObject for SSubVector {
    fn getClass<'a>(&self) -> &'a clojure::rust::Class::SClass {
        todo!()
    }

    fn hashCode(&self) -> usize {
        todo!()
    }

    fn equals(&self, other: &Object) -> bool {
        todo!()
    }

    fn toString(&self) -> usize {
        todo!()
    }
}
