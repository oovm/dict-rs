use crate::StringMap;
use std::{
    any::Any,
    ops::{Index, IndexMut},
};

impl Default for StringMap {
    fn default() -> Self {
        StringMap(Default::default())
    }
}

impl Index<&str> for StringMap {
    type Output = Box<dyn Any + Send + Sync>;

    fn index(&self, key: &str) -> &Self::Output {
        self.0.index(key)
    }
}
/*
impl IndexMut<&str> for StringMap {
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        &mut self.index(key)
    }
}
*/