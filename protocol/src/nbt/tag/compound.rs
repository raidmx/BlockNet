use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use super::Tag;

/// CompoundTag represents a Tag compound which contains a mapping of Key-Value based values.
/// Key is always of type String and Value is of type Tag.
#[derive(PartialEq, Clone)]
pub struct CompoundTag<'a> {
    map: HashMap<String, Tag<'a>>,
}

impl<'a> CompoundTag<'a> {
    /// Creates and returns a new Compound
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Creates and returns a new Compound Tag with capacity
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            map: HashMap::with_capacity(cap),
        }
    }

    /// Gets a reference to the value at provided key.
    pub fn get(&self, key: &str) -> Option<&Tag> {
        let val = self.map.get(key)?;
        Some(val)
    }

    /// Gets a mutable reference to the value at provided key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Tag<'a>> {
        let val = self.map.get_mut(key)?;
        Some(val)
    }

    /// Puts the provided Tag object into the map at provided key.
    pub fn put(&mut self, key: &str, tag: Tag<'a>) {
        self.map.insert(key.to_string(), tag);
    }
}

/*
    Creates and returns a Compound Tag. Provided below is an example use case.

    # Example

    ```
    let compound = compound![
        "byte" => 120_i8,
        "map" => compound!(
            "integer" => 100_i32,
            "string" => "hello world"
        )
    ];
    ```
*/
#[macro_export]
macro_rules! compound {
    ($($key:expr => $value:expr),*) => {{
        let mut map = CompoundTag::new();
        $(
            map.put($key, $value.into());
        )*
        Tag::Compound(map)
    }};
}

impl<'a> Debug for CompoundTag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.map)
    }
}

impl<'a> Deref for CompoundTag<'a> {
    type Target = HashMap<String, Tag<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<'a> DerefMut for CompoundTag<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}