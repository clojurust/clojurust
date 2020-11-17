//! clojure::rust::keyword: keyword index

use im::vector::Vector;
use im::hashmap::HashMap;
use std::result::Result;

struct Keyword {
    data: &'static mut Vector<&'static str>,
}

impl<'i> Keyword {
    fn get( self, index: usize) -> Option<&'i &'static str> {
        self.data.get(index)
    }

    fn len(self) -> usize {
        self.data.len()
    }

    fn new() -> Keyword {
        Keyword {
            data: Vector::<&'static str>::new(),
        }
    }

    fn push_back(&mut self, keyword: &'static str) -> usize {
        self.data.push_back(keyword);
        self.len()
    }
}

struct KeyMap {
    data: &'static mut HashMap<&'static str, usize>
}

impl<'i> KeyMap {
    fn get(self, key: &'static str) -> Option<&usize> {
        self.data.get(key)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn new() -> KeyMap {
        KeyMap {
            data: &mut HashMap::<&'static str, usize>::new(),
        }
    }

    /// Insert new key 
    /// If it doesn't exists add keyword
    fn insert(&mut self, key: &'static str) -> usize {
        if let Some(index) = self.get(key) {
            // exists
            *index
        }
        else
        {
            // doesn't exists
            STATE.keyword.push_back(key)
        }
    } 
}

struct State {
    keyword: Keyword,
    key_map: KeyMap,
}

static STATE: State = State {
    keyword: Keyword::new(),
    key_map: KeyMap::new(),
};

fn init_static() {
    println!("init static");
}

#[allow(dead_code)]
fn add_keyword(keyword: &'static str) -> usize {
    println!("Add keyword");
    if let Some(index) = STATE.key_map.get(keyword) {
        println!("Already exists");
        *index
    }
    else {
        println!("Don't exists");
        let index = STATE.keyword.len();
        println!("len = {}", index);
        let us = STATE.key_map.insert(keyword);
        println!("got new index : {}", us);
        us
    }
}

#[test]
fn test_the_thing() -> Result<(), ()> {
    println!("Go init_static");
    init_static(); // expected to succeed
    println!("Go add_keyword");
    let a = add_keyword(r#"fdsafdsafdsa"#);
    println!("index = {}", a);
    Ok(())
}
