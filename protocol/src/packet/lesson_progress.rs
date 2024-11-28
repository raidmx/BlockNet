use binary::v32;
use derive::{Decode, Encode, Packet};

/// Sent by the server to the client to inform the client of updated progress on a lesson. This
/// packet only functions on the Minecraft: Education Edition version of the game.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct LessonProgress {
    /// The action the client should perform to show progress.
    pub action: LessonAction,
    /// The score the client should use when displaying the progress.
    pub score: v32,
    /// The identifier of the lesson that is being progressed.
    pub identifier: String,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum LessonAction {
    Start,
    Complete,
    Restart,
}
