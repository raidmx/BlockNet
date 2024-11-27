use bytes::Bytes;
use derive::{Decode, Encode, Packet};

/// Education Edition packet sent by the client to run an operation with a code builder.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct CodeBuilderSource {
    /// The operation to be performed.
    pub operation: CodeBuilderOperation,
    /// The category in which the operation falls under.
    pub category: CodeBuilderCategory,
    /// Extra data about the operation performed. It is always empty unless the operation is set.
    pub value: Bytes,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum CodeBuilderCategory {
    None,
    Status,
    Instantiation,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum CodeBuilderOperation {
    None,
    Get,
    Set,
    Reset,
}
