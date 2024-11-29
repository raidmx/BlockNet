use uuid::Uuid;
use binary::{v64, Decode, Encode, Reader, Writer};
use derive::{Decode, Encode, Packet};
use crate::types::device::Device;
use crate::types::skin::Skin;

#[derive(Clone, Debug, Encode, Decode)]
#[encoding(type = u8)]
pub enum PlayerListAction<'a> {
    Add(PlayerListAdd<'a>),
    Remove(PlayerListRemove),
}

/// Sent by the server to update the client-side player list in the in-game menu screen. It shows
/// the icon of each player if the correct XUID is written in the packet. Sending the PlayerList
/// packet is obligatory when sending an AddPlayer packet. The added player will not show up to a
/// client if it has not been added to the player list, because several properties of the player are
/// obtained from the player list, such as the skin.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PlayerList<'a> {
    /// The action to execute upon the player list. The entries that are contained specify which
    /// entries are added or removed from the player list.
    pub action_type: PlayerListAction<'a>,
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct PlayerListRemove {
    /// A list of UUIDs to remove.
    pub uuids: Vec<Uuid>,
}

#[derive(Clone, Debug)]
pub struct PlayerListAdd<'a> {
    pub entries: Vec<PlayerListEntry<'a>>,
}

impl<'a> Encode for PlayerListAdd<'a> {
    fn encode(&self, w: &mut Writer) {
        self.entries.encode(w);

        for entry in &self.entries {
            entry.skin.trusted.encode(w);
        }
    }
}

impl<'a> Decode<'a> for PlayerListAdd<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let mut entries: Vec<PlayerListEntry> = Vec::decode(r)?;

        for i in 0..entries.len() {
            entries[i].skin.trusted = bool::decode(r)?;
        }

        Some(PlayerListAdd { entries })
    }
}

/// An entry found in the PlayerList packet. It represents a single player using the UUID found in
/// the entry, and contains several properties such as the skin.
#[derive(Debug, Clone, Encode, Decode)]
pub struct PlayerListEntry<'a> {
    /// The UUID of the player as sent in the Login packet when the client joined the server. It
    /// must match this UUID exactly for the correct XBOX Live icon to show up in the list.
    pub uuid: Uuid,
    /// The unique entity ID of the player. This ID typically stays consistent during the lifetime
    /// of a world, but servers often send the runtime ID for this.
    pub entity_unique_id: v64,
    /// The username that is shown in the player list of the player that obtains a PlayerList packet
    /// with this entry. It does not have to be the same as the actual username of the player.
    pub username: &'a str,
    /// The XBOX Live user ID of the player, which will remain consistent as long as the player is
    /// logged in with the XBOX Live account.
    pub xuid: &'a str,
    /// An identifier only set for particular platforms when chatting (presumably only for Nintendo
    /// Switch). It is otherwise an empty string, and is used to decide which players are able to
    /// chat with each other.
    pub platform_chat_id: &'a str,
    /// The platform of the player as sent by that player in the Login packet.
    pub build_platform: Device,
    /// The skin of the player that should be added to the player list. Once sent here, it will not
    /// have to be sent again.
    pub skin: Skin<'a>,
    /// Minecraft: Education Edition field. It specifies if the player to be added to the player
    /// list is a teacher.
    pub teacher: bool,
    /// Specifies if the player that is added to the player list is the host of the game.
    pub host: bool,
}
