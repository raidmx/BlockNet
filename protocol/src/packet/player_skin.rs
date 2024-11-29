use uuid::Uuid;
use binary::{Decode, Encode, Reader, Writer};
use derive::Packet;

use crate::types::skin::Skin;

/// Sent by the client to the server when it updates its own skin using the in-game skin picker. It
/// is relayed by the server, or sent if the server changes the skin of a player on its own accord.
/// Note that the packet can only be sent for players that are in the player list.
#[derive(Debug, Clone, Packet)]
pub struct PlayerSkin<'a> {
    /// The UUID of the player as sent in the Login packet when the client joined the server. It
    /// must match this UUID exactly for the skin to show up on the player.
    pub uuid: Uuid,
    /// The new skin to be applied on the player with the UUID in the field above. The skin,
    /// including its animations, will be shown after sending it.
    pub skin: Skin<'a>,
    /// No longer has a function. The field can be left empty at all times.
    pub new_skin_name: &'a str,
    /// No longer has a function. The field can be left empty at all times.
    pub old_skin_name: &'a str,
}

impl<'a> Encode for PlayerSkin<'a> {
    fn encode(&self, w: &mut Writer) {
        self.uuid.encode(w);
        self.skin.encode(w);
        self.new_skin_name.encode(w);
        self.old_skin_name.encode(w);
        self.skin.trusted.encode(w);
    }
}

impl<'a> Decode<'a> for PlayerSkin<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let mut pk = Self {
            uuid: Uuid::decode(r)?,
            skin: Skin::decode(r)?,
            new_skin_name: <&'a str>::decode(r)?,
            old_skin_name: <&'a str>::decode(r)?,
        };

        pk.skin.trusted = bool::decode(r)?;
        Some(pk)
    }
}