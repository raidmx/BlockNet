use binary::{v32, Decode, Encode, Reader, Writer};
use derive::Packet;

/// Sent by the server to disconnect the client using an optional message to send as the disconnect
/// screen.
#[derive(Debug, Clone, Default, Packet)]
pub struct Disconnect<'a> {
    /// The reason why the user was kicked. Used for telemetry.
    pub reason: v32,
    /// This specifies if the disconnection screen should be hidden when the client is
    /// disconnected, meaning it will be sent directly to the main menu.
    pub hide_disconnection_screen: bool,
    /// An optional message to show when disconnected. If left empty, this message is only written
    /// if the hide_disconnection_screen field is set to true.
    pub message: &'a str,
    /// An optional field which is always set to empty and the usage is currently unknown.
    pub filtered_message: &'a str
}

impl<'a> Encode for Disconnect<'a> {
    fn encode(&self, w: &mut Writer) {
        self.reason.encode(w);
        self.hide_disconnection_screen.encode(w);

        if !self.hide_disconnection_screen {
            self.message.encode(w);
            self.filtered_message.encode(w);
        }
    }
}

impl<'a> Decode<'a> for Disconnect<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let mut pk = Self {
            reason: v32::decode(r)?,
            hide_disconnection_screen: bool::decode(r)?,
            ..Default::default()
        };
        
        if !pk.hide_disconnection_screen {
            pk.message = <&'a str>::decode(r)?;
            pk.filtered_message = <&'a str>::decode(r)?;
        }
        
        Some(pk)
    }
}
