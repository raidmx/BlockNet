use binary::v32;
use derive::{Decode, Encode, Packet};
use crate::types::BlockPos;

/// The purpose of this packet is currently unknown.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct GameTestRequest {
    /// The purpose of this field is currently unknown.
    pub max_tests_per_batch: v32,
    /// The amount of times the test will be run.
    pub repetitions: v32,
    /// The rotation of the test.
    pub rotation: GameTestRequestRotation,
    /// Indicates whether the test should immediately stop when an error is encountered.
    pub stop_on_error: bool,
    /// The position at which the test will be performed.
    pub position: BlockPos,
    /// The purpose of this field is currently unknown.
    pub tests_per_row: v32,
    /// The name of the test.
    pub name: String,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum GameTestRequestRotation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    Rotate360,
}
