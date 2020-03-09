use crate::utils::force_downcast;
use std::{
    any::{type_name, Any},
    collections::{
        hash_map::{Entry, Iter, Keys, Values, ValuesMut},
        HashMap,
    },
};

pub struct StringMap(pub(crate) HashMap<Box<str>, (Box<dyn Any + Send + Sync>, Box<str>)>);

impl StringMap {
    pub fn insert<T: Any + Send + Sync>(&mut self, key: &str, x: T) -> Option<T> {
        match self.0.insert(Box::from(key), (Box::new(x), Box::from(type_name::<T>()))) {
            Some((v, _)) => Some(force_downcast(v)),
            None => None,
        }
    }

    pub fn remove<T: Any + Send + Sync>(&mut self, key: &str) -> Option<T> {
        match self.0.remove(key) {
            Some((v, _)) => Some(force_downcast(v)),
            None => None,
        }
    }

    pub fn get<T: Any + Send + Sync>(&self, key: &str) -> Option<&T> {
        if let Some((v, _)) = self.0.get(key) {
            if let Some(s) = v.downcast_ref::<T>() {
                return Some(s);
            }
        }
        return None;
    }

    pub fn get_mut<T: Any + Send + Sync>(&mut self, key: &str) -> Option<&mut T> {
        if let Some((v, _)) = self.0.get_mut(key) {
            if let Some(s) = v.downcast_mut::<T>() {
                return Some(s);
            }
        }
        return None;
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
        self.0.keys();
        unimplemented!()
    }
    pub fn values(&self) -> Values<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.values();
        unimplemented!()
    }

    pub fn iter(&self) -> Iter<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.iter();
        unimplemented!()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
    pub fn values_mut(&mut self) -> ValuesMut<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.values_mut();
        unimplemented!()
    }

    pub fn entry(&mut self, key: &str) -> Entry<Box<str>, Box<dyn Any + Send + Sync>> {
        self.0.entry(Box::from(key));
        unimplemented!()
    }
}
