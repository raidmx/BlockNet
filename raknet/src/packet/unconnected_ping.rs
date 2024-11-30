use binary::b64;
use crate::types::Magic;

pub struct UnconnectedPing {
    pub ping_time: b64,
    pub magic: Magic,
    pub client_guid: b64
}