use crate::nbt::{Tag, TagId};

/// List represents a collection of Tag objects. It is a homogenous collection of Tag objects, in other
/// words, objects of same type only.
pub type List<'a> = Vec<Tag<'a>>;

/// Returns the TagId of elements contained inside the List.
pub fn get_list_type(list: &List) -> TagId{
    if list.len() == 0 {
        return TagId::End;
    }

    list[0].id()
}

/*
    Creates and returns a List Tag. Provided below is an example use case.

    # Example

    ```
    To create a list of short i16:
    list![1_i16, 2_i16, 3_i16];
    ```
*/
#[macro_export]
macro_rules! list {
    ($($value:expr),*) => {{
        let mut list = List::new();
        $(
            list.push($value.into());
        )*
        list
    }};
}