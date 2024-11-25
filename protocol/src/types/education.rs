use derive::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct EducationExternalLinkSettings {
    pub url: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct EducationSharedResourceURI {
    pub button_name: String,
    pub link_uri: String,
}
