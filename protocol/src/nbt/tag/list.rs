use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::nbt::{Tag, TagId};

/// List represents a collection of Tag objects. It is a homogenous collection of Tag objects, in other
/// words, objects of same type only.
#[derive(Default, PartialEq, Clone)]
pub struct ListTag<'a> {
    id: TagId,
    list: Vec<Tag<'a>>,
}

impl<'a> ListTag<'a> {
    /// Creates and returns a new List object with the provided type of objects
    pub fn new(id: TagId) -> Self {
        Self {
            id,
            list: Vec::new(),
        }
    }

    /// Creates and returns a new List object with the provided type and the specified
    /// capacity.
    pub fn with_capacity(id: TagId, cap: usize) -> Self {
        Self {
            id,
            list: Vec::with_capacity(cap),
        }
    }

    /// Returns the type of Tags contained in the List
    pub fn list_type(&self) -> TagId {
        self.id
    }

    /// Gets a reference to the value at provided index.
    pub fn get(&self, index: usize) -> Option<&Tag> {
        let val = self.list.get(index)?;
        Some(val)
    }

    /// Gets a mutable reference to the value at provided index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Tag<'a>> {
        let val = self.list.get_mut(index)?;
        Some(val)
    }

    /// Puts the provided Tag object at the end of the list.
    pub fn put(&mut self, tag: Tag<'a>) {
        if self.id != tag.id() {
            return;
        }

        self.list.push(tag)
    }

    /// Inserts the provided Tag object at the provided index.
    pub fn insert(&mut self, index: usize, tag: Tag<'a>) {
        if self.id != tag.id() {
            return;
        }

        self.list.insert(index, tag)
    }
}

/*
    Creates and returns a List Tag. Provided below is an example use case.

    # Example

    ```
    list![TagId::Short, 1, 2, 3];
    ```
*/
#[macro_export]
macro_rules! list {
    ($TAG_ID:expr, $($TAGS:expr),*) => {{
        let mut list = ListTag::new($TAG_ID);
        $(list.list.push($TAGS.into());)*
        Tag::List(list)
    }};
}

impl<'a> Debug for ListTag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.list)
    }
}

impl<'a> Deref for ListTag<'a> {
    type Target = Vec<Tag<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl<'a> DerefMut for ListTag<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}