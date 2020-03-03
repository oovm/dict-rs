# StringMap

Create a record to store any type of value

## Basic Usage

```rust
extern crate string_map;

use string_map::StringMap;

#[test]
fn main() {
    let mut dict = StringMap::new();
    dict.insert("number", 0);
    dict.insert("string", "str");
    dict.insert("boolean", true);

    debug_assert_eq!(dict.get("number"), Some(&0));
    debug_assert_eq!(dict.get_mut("string"), Some(&mut "str"));
    debug_assert_eq!(dict.get_key_value("boolean"), Some(("boolean", &true)));
    debug_assert_eq!(dict.remove::<bool>("nothing"), None);
}
```