pub mod list;
pub mod compound;

pub use list::*;
pub use compound::*;

macro_rules! impl_tag {
    ($variant:ident, $type:ty, $as:ident, $as_mut:ident) => {
        impl<'a> Tag<'a> {
            pub fn $as(&self) -> &$type {
                match self {
                    Tag::$variant(val) => val,
                    _ => panic!("Cannot convert Tag object to inner type"),
                }
            }

            pub fn $as_mut(&mut self) -> &mut $type {
                match self {
                    Tag::$variant(val) => val,
                    _ => panic!("Cannot convert Tag object to inner type"),
                }
            }
        }

        impl<'a> From<$type> for Tag<'a> {
            fn from(value: $type) -> Self {
                Tag::$variant(value)
            }
        }

        impl<'a> From<Tag<'a>> for $type {
            fn from(value: Tag<'a>) -> $type {
                match value {
                    Tag::$variant(val) => val,
                    _ => panic!("Cannot convert Tag object to inner type"),
                }
            }
        }
    };
}

macro_rules! generate_tags {
    ($($variant:ident => $type:ty, $as:ident, $as_mut:ident),*) => {
        /// TagId is an enumeration of Tag Ids for different types of Tags.
        #[derive(Default, Debug, PartialEq, Clone, Copy)]
        #[repr(u8)]
        pub enum TagId {
            #[default]
            End,
            $($variant,)*
        }

        impl TagId {
            pub fn from_byte(byte: u8) -> Option<Self> {
                match byte {
                    0 => Some(Self::End),
                    $(x if x == TagId::$variant as u8 => Some(Self::$variant),)*
                    _ => None,
                }
            }
        }

        /// Tag is an implementation of NBT Tag. Each Tag Object has a unique identifier [`TagId`] associated
        /// with it.
        #[derive(Default, PartialEq, Clone)]
        pub enum Tag<'a> {
            #[default]
            End,
            $($variant($type),)*
        }

        impl<'a> Tag<'a> {
            pub fn id(&self) -> TagId {
                match self {
                    $(Self::$variant(_) => TagId::$variant,)*
                }
            }
        }

        impl<'a> std::fmt::Debug for Tag<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Tag::$variant(value) => write!(f, "{:?}", value),)*
                }
            }
        }

        $(
            impl_tag!($variant, $type, $as, $as_mut);
        )*
    };
}

generate_tags!(
    Byte => i8, as_byte, as_mut_byte,
    Short => i16, as_short, as_mut_short,
    Int => i32, as_int, as_mut_int,
    Long => i64, as_long, as_mut_long,
    Float => f32, as_float, as_mut_float,
    Double => f64, as_double, as_mut_double,
    ByteArray => &'a [i8], as_byte_array, as_mut_byte_array,
    String => &'a str, as_string, as_mut_string,
    List => ListTag<'a>, as_list, as_mut_list,
    Compound => CompoundTag<'a>, as_compound, as_mut_compound,
    IntArray => Vec<i32>, as_int_array, as_mut_int_array,
    LongArray => Vec<i64>, as_long_array, as_mut_long_array
);