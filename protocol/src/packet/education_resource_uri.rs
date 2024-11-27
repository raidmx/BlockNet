use derive::{Decode, Encode, Packet};

use crate::types::education::EducationSharedResourceURI;

/// Transmits education resource settings to all clients.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct EducationResourceURI {
    /// The resource that is being referenced.
    pub resource: EducationSharedResourceURI,
}
