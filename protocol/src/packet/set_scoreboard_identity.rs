use binary::{w32, Decode, Encode, Reader, Writer, Numeric};
use derive::Packet;
use crate::types::scoreboard::{ScoreboardIdentityAction, ScoreboardIdentityEntry};

/// Sent by the server to change the identity type of one of the entries on a scoreboard. This is
/// used to change, for example, an entry pointing to a player, to a fake player when it leaves the
/// server, and to change it back to a real player when it joins again. In non-vanilla situations,
/// the packet is quite useless.
#[derive(Debug, Clone, Packet)]
pub struct SetScoreboardIdentity {
    /// The type of the action to execute. The action is either `Register` to associate an identity
    /// with the entry, or `Clear` to remove associations with an entity.
    pub action_type: ScoreboardIdentityAction,
    /// A list of all entries in the packet. Each of these entries points to one of the entries on
    /// a scoreboard. Depending on `action_type`, it'll either be registered or cleared.
    pub entries: Vec<ScoreboardIdentityEntry>,
}

impl<'a> Encode for SetScoreboardIdentity {
    fn encode(&self, w: &mut Writer) {
        self.action_type.encode(w);

        w32::from_usize(self.entries.len()).encode(w);

        self.entries
            .iter()
            .for_each(|entry| entry.write(w, &self.action_type));
    }
}

impl Decode<'_> for SetScoreboardIdentity {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let action_type = ScoreboardIdentityAction::decode(r)?;

        let len = w32::decode(r)?.to_usize();
        let entries: Vec<_> = (0..len).filter_map(|_| ScoreboardIdentityEntry::read(r, &action_type)).collect();

        Some(Self {
            action_type,
            entries
        })
    }
}
