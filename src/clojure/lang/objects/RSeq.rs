use clojure::lang::*;
use clojure::rust::*;
use intertrait::*;

use crate::*;
castable_to!(SRSeq => [sync] IObject, ASeq, IndexedSeq, Counted);

pub struct SRSeq {
    v: Object, //&'a IPersistantVector,
    i: usize,
}

pub trait RSeq: IObject+ASeq+IndexedSeq+Counted {}

impl IObject for SRSeq {
    fn getClass<'a>(&self) -> &'a SClass { todo!() }

    fn hashCode(&self) -> usize { todo!() }

    fn toString(&self) -> String { todo!() }

    fn equals(
        &self,
        other: &Object,
    ) -> bool {
        todo!()
    }
}

impl IObj for SRSeq {
    fn withMeta(
        &self,
        meta: &Object,
    ) -> ObjResult<Object> {
        todo!()
    }
}

impl ASeq for SRSeq {}

impl IMeta for SRSeq {
    fn meta(&self) -> ObjResult<Object> { todo!() }
}

impl IHashEq for SRSeq {
    fn hasheq(&self) -> ObjResult<usize> { todo!() }
}

impl Serializable for SRSeq {}

impl Sequential for SRSeq {}

impl Counted for SRSeq {
    fn count(&self) -> ObjResult<usize> { todo!() }
}

impl IndexedSeq for SRSeq {
    fn index(&self) -> ObjResult<usize> { todo!() }
}

impl Sequable for SRSeq {
    fn seq(&self) -> ObjResult<Object> { todo!() }
}

impl Iterable for SRSeq {
    fn iterator(&self) -> ObjResult<Object> { todo!() }
}

impl ISeq for SRSeq {
    fn cons(
        &self,
        o: &Object,
    ) -> ObjResult<Object> {
        todo!()
    }
}

impl List for SRSeq {}

impl IPersistentCollection for SRSeq {
    fn cons(
        &self,
        o: &Object,
    ) -> ObjResult<Object> {
        todo!()
    }

    fn count(&self) -> ObjResult<usize> { todo!() }

    fn empty(&self) -> ObjResult<Object> { todo!() }

    fn equiv(
        &self,
        o: Object,
    ) -> ObjResult<bool> {
        todo!()
    }
}

impl Collection for SRSeq {
    fn size(&self) -> usize { todo!() }

    fn isEmpty(&self) -> bool { todo!() }

    fn contains(&self) -> ObjResult<bool> { todo!() }

    fn toArray(&self) -> ObjResult<Vec<Object>> { todo!() }

    fn containsAll(
        &self,
        c: &Object,
    ) -> ObjResult<bool> {
        todo!()
    }
}
