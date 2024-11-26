use num_derive::{FromPrimitive, ToPrimitive};
use derive::{Decode, Encode};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = u8)]
pub enum ResourcePackResponse {
    None,
    Refused,
    SendPacks,
    AllPacksDownloaded,
    Completed,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = u8)]
pub enum ResourcePackType {
    Addon,
    Cached,
    CopyProtected,
    Behaviour,
    PersonaPiece,
    Resources,
    Skins,
    WorldTemplate,
}
