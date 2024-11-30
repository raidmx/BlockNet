use std::collections::HashMap;
use binary::generate;
use super::Tag;

generate!(Compound, <>, HashMap<String, Tag<'a>>, 'a);

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
        let mut map = Compound::new();
        $(
            map.insert($key, $value.into());
        )*
        Tag::Compound(map)
    }};
}