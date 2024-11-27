use derive::{Decode, Encode, Packet};

/// Sent in response to the GameTestRequest packet, with a boolean indicating whether the test was
/// successful or not, and an error string if the test failed.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct GameTestResults {
    /// The name of the test.
    pub name: String,
    /// Indicates whether the test succeeded or not.
    pub succeeded: bool,
    /// The error that occurred. If succeeded is true, this field is empty.
    pub error: String,
}
