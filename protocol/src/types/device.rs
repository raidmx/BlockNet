#![allow(deprecated)]

use derive::{Decode, Encode};

#[derive(Debug, Copy, Clone, Encode, Decode)]
#[encoding(type = i32)]
pub enum Device {
    Android = 1,
    IOS,
    OSX,
    FireOS,
    GearVR,
    Hololens,
    Win10,
    Win32,
    Dedicated,
    #[deprecated = "Deprecated as of Bedrock Edition v1.20.10"]
    TVOS,
    Orbis,
    NX,
    XBOX,
    #[deprecated = "Deprecated as of Bedrock Edition v1.20.10"]
    WP,
    Linux,
    Unknown,
}
