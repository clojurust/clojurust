//! # HashMap of `Object`s with `IObject` protocol
//!
//! This is a wrapper on `im-rs` HashMap<Object,Object> library

// use intertrait::cast::*;

use clojure::lang::*;
use clojure::rust::*;
use im::hashmap::*;
use intertrait::*;

use crate::*;
castable_to!(SPersistentHashMap => [sync] Associative, Callable, Counted,  IEditableCollection, IFn, IObj, IObject, IMeta, Iterable, IHashEq,  IKVReduce, ILookup, IMapIterable, IPersistentCollection, IPersistentMap,   Map, MapEquivalence, Runnable, Sequable, Serializable);

pub struct SPersistentHashMap
{
    inner: HashMap<Object, Object>,
}

pub trait PersistentHashMap:
    IObject+IEditableCollection+IObj+IMapIterable+IKVReduce
{
}

impl SPersistentHashMap {}

impl IPersistentMap for SPersistentHashMap
{
    fn assoc(
        &self,
        key: Object,
        val: Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }

    #[allow(non_snake_case)]
    fn assocEx(
        &self,
        key: Object,
        val: Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }

    fn without(
        &self,
        key: Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }
}

impl Counted for SPersistentHashMap
{
    fn count(&self) -> ObjResult<usize> { todo!() }
}

impl IMeta for SPersistentHashMap
{
    fn meta(&self) -> ObjResult<Object> { todo!() }
}

impl Iterable for SPersistentHashMap
{
    fn iterator(&self) -> ObjResult<Object> { todo!() }
}

impl Associative for SPersistentHashMap
{
    fn assoc(
        &self,
        key: &Object,
        value: &Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }

    #[allow(non_snake_case)]
    fn containsKey(
        &self,
        key: &Object,
    ) -> ObjResult<bool>
    {
        todo!()
    }

    #[allow(non_snake_case)]
    fn entryAt(
        &self,
        key: &Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }
}

impl IObject for SPersistentHashMap
{
    #[allow(non_snake_case)]
    fn getClass<'a>(&self) -> &'a SClass { todo!() }

    #[allow(non_snake_case)]
    fn hashCode(&self) -> usize { todo!() }

    fn equals(
        &self,
        other: &Object,
    ) -> bool
    {
        todo!()
    }

    #[allow(non_snake_case)]
    fn toString(&self) -> String { todo!() }
}

impl Default for SPersistentHashMap
{
    fn default() -> Self
    {
        SPersistentHashMap {
            inner: HashMap::<Object, Object>::default(),
        }
    }
}

impl PersistentHashMap for SPersistentHashMap {}

impl Iterator for SPersistentHashMap
{
    type Item = SPersistentHashMap;

    fn next(&mut self) -> Option<Self::Item> { todo!() }
}

impl IMapIterable for SPersistentHashMap
{
    type Item = SPersistentHashMap;

    fn keyIterator(&self) -> ObjResult<Object> { todo!() }

    fn valIterator(&self) -> ObjResult<Object> { todo!() }
}

impl IObj for SPersistentHashMap
{
    fn withMeta(
        &self,
        meta: &Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }
}

impl Callable for SPersistentHashMap
{
    fn call(&self) -> ObjResult<Object> { self.call() }
}

impl IFn for SPersistentHashMap
{
    fn invoke(
        &self,
        args: &[&Object],
    ) -> ObjResult<Object>
    {
        todo!()
    }

    fn apply(
        &self,
        arglist: &ISeq,
    ) -> ObjResult<Object>
    {
        todo!()
    }
}

impl IHashEq for SPersistentHashMap
{
    fn hasheq(&self) -> ObjResult<usize> { todo!() }
}

impl ILookup for SPersistentHashMap
{
    fn valAt1(
        &self,
        key: Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }

    fn valAt2(
        &self,
        key: Object,
        notFound: Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }
}

impl IPersistentCollection for SPersistentHashMap
{
    fn cons(
        &self,
        o: &Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }

    fn count(&self) -> ObjResult<usize> { todo!() }

    fn empty(&self) -> ObjResult<Object> { todo!() }

    fn equiv(
        &self,
        o: Object,
    ) -> ObjResult<bool>
    {
        todo!()
    }
}

impl Sequable for SPersistentHashMap
{
    fn seq(&self) -> ObjResult<Object> { todo!() }
}

impl MapEquivalence for SPersistentHashMap {}

impl Map for SPersistentHashMap {}

impl Runnable for SPersistentHashMap
{
    fn run(&self) { todo!() }
}

impl Thread for SPersistentHashMap {}

impl Serializable for SPersistentHashMap {}

impl IEditableCollection for SPersistentHashMap
{
    fn asTransient(&self) -> ObjResult<Object> { todo!() }
}

impl IKVReduce for SPersistentHashMap
{
    fn kvreduce(
        &self,
        f: Object,
        init: Object,
    ) -> ObjResult<Object>
    {
        todo!()
    }
}
