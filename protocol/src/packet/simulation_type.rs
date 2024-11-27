use derive::{Decode, Encode, Packet};

/// An in-progress packet. We currently do not know the use case.\
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SimulationType {
    /// The simulation type selected.
    pub simulation_type: Simulation,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum Simulation {
    Game,
    Editor,
    Test,
    Invalid,
}
