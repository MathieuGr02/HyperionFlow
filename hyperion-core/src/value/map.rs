use std::{
    borrow::Borrow,
    collections::{
        HashMap,
        hash_map::{IntoIter, Iter},
    },
    hash::Hash,
};

use delegate::delegate;

use crate::value::value::Value;

#[derive(Debug)]
pub struct Map {
    inner: HashMap<String, Value>,
}

impl From<HashMap<String, Value>> for Map {
    fn from(value: HashMap<String, Value>) -> Self {
        Map { inner: value }
    }
}

impl Into<Value> for Map {
    fn into(self) -> Value {
        Value::Map(self)
    }
}

impl Map {
    pub fn new() -> Self {
        Map {
            inner: HashMap::new(),
        }
    }

    delegate! {
        to self.inner {
            pub fn contains_key(&self, k: &String) -> bool;
            pub fn get<Q>(&self, k: &Q) -> Option<&Value> where Q: Hash + Eq + ?Sized, String: Borrow<Q>;
            pub fn get_mut(&mut self, k: &String) -> Option<&mut Value>;
            pub fn insert(&mut self, k: String, v: Value) -> Option<Value>;
            pub fn iter(&self) -> Iter<'_, String, Value>;
            pub fn remove<Q>(&mut self, k: &Q) -> Option<Value> where Q: Hash + Eq + ?Sized, String: Borrow<Q>;
            pub fn len(&self) -> usize;
        }
    }
}

impl IntoIterator for Map {
    type Item = (String, Value);
    type IntoIter = IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
