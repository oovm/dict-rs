use crate::StringMap;
use std::fmt::{self, Debug};

impl Default for StringMap {
    fn default() -> Self {
        StringMap(Default::default())
    }
}

impl Debug for StringMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.0.iter().map(|(k, v)| (k, &v.1))).finish()
    }
}
/*
impl<T> Index<&str,T> for StringMap {
    type Output = T;

    fn index(&self, key: &str) -> &Self::Output {
        self.get::<T>(key).unwrap()
    }
}
*/
/*
impl IndexMut<&str> for StringMap {
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        &mut self.index(key)
    }
}
*/
