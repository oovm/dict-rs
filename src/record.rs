use crate::utils::force_downcast;
use std::{
    any::Any,
    collections::{
        hash_map::{Entry, Iter, Keys, Values, ValuesMut},
        HashMap,
    },
};

pub struct StringMap(pub(crate) HashMap<Box<str>, Box<dyn Any + Send + Sync>>);

impl StringMap {
    pub fn insert<T: Any + Send + Sync>(&mut self, key: &str, x: T) -> Option<T> {
        self.0.insert(Box::from(key), Box::new(x)).map(force_downcast)
    }

    pub fn remove<T: Any + Send + Sync>(&mut self, key: &str) -> Option<T> {
        self.0.remove(key).map(force_downcast)
    }

    pub fn get<T: Any + Send + Sync>(&self, key: &str) -> Option<&T> {
        self.0.get(key).map(|b| b.downcast_ref::<T>().unwrap())
    }

    pub fn get_mut<T: Any + Send + Sync>(&mut self, key: &str) -> Option<&mut T> {
        self.0.get_mut(key).map(|b| b.downcast_mut::<T>().unwrap())
    }

    pub fn get_key_value<'s, T: Any + Send + Sync>(&self, key: &'s str) -> Option<(&'s str, &T)> {
        match self.get(key) {
            None => None,
            Some(v) => Some((key, v)),
        }
    }
}

impl StringMap {
    pub fn new() -> Self {
        StringMap::default()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn keys(&self) -> Keys<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.keys()
    }
    pub fn values(&self) -> Values<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.values()
    }

    pub fn iter(&self) -> Iter<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.iter()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
    pub fn values_mut(&mut self) -> ValuesMut<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.values_mut()
    }

    pub fn entry(&mut self, key: &str) -> Entry<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.entry(Box::from(key))
    }
}
