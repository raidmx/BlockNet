use binary::VarI32;
use derive::{Decode, Encode, Packet};

/// Sent from the server to the client to change one of the properties of a mob client-side.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ChangeMobProperty {
    /// The unique ID of the entity whose property is being changed.
    pub entity_unique_id: u64,
    /// The name of the property being updated.
    pub property: String,
    /// Set if the property value is a bool. If the type is not a bool, this field is ignored.
    pub bool_value: bool,
    /// Set if the property value is a string. If the type is not a string, this field is ignored.
    pub string_value: String,
    /// Set if the property value is an int. If the type is not an int, this field is ignored.
    pub int_value: VarI32,
    /// Set if the property value is a float. If the type is not a float, this field is ignored.
    pub float_value: f32,
}
