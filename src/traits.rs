use crate::StringMap;
use std::{any::Any, ops::Index};

impl Default for StringMap {
    fn default() -> Self {
        StringMap(Default::default())
    }
}

/*
impl Index<&str> for StringMap
where {
    type Output = Box<dyn Any + Send + Sync>;

    fn index(&self, key: &str) -> &Self::Output {
         self.0.get(key).unwrap()
    }
}
*/
