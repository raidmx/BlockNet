use std::collections::HashMap;
use super::Tag;

/// Compound represents a heterogeneous collection of objects indexed by string keys.
pub type Compound<'a> = HashMap<&'a str, Tag<'a>>;

/*
    Creates and returns a Compound Tag. Provided below is an example use case.

    # Example

    ```
    let compound = compound! {
        "byte" => 120_i8,
        "map" => compound! {
            "integer" => 100_i32,
            "string" => "hello world"
        }
    };
    ```
*/

#[macro_export]
macro_rules! compound {
    ($($key:expr => $value:expr),*) => {{
        let mut compound = Compound::new();
        $(
            compound.insert($key, $value.into());
        )*
        compound
    }};
}