use derive::{Decode, Encode, Packet};

#[derive(Debug, Encode, Decode, Packet)]
pub struct DetectLostConnections;