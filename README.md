# StringMap

Create a record to store any type of value

## Basic Usage

```rs
extern crate string_map;

use string_map::StringMap;

#[test]
fn main() {
    let mut dict = StringMap::new();
    dict.insert("number", 0);
    dict.insert("string", "str");
    dict.insert("boolean", true);

    assert_eq!(dict.get("number"), Some(&0));
    assert_eq!(dict.get_mut("string"), Some(&mut "str"));
    assert_eq!(dict.remove("boolean"), Some(true));
}
```