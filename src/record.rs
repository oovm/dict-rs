use crate::utils::force_downcast;
use std::{any::Any, collections::HashMap};

pub struct StringMap(HashMap<Box<str>, Box<dyn Any + Send + Sync>>);

impl Default for StringMap {
    fn default() -> Self {
        StringMap(Default::default())
    }
}

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
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn clear(&mut self) {
        self.0.clear()
    }
}
