use binary::{w32, Decode, Encode, Reader, Writer, Numeric};
use derive::Packet;
use crate::types::scoreboard::{ScoreboardAction, ScoreboardEntry};

/// Sent by the server to send the contents of a scoreboard to the player. It may be used to either
/// add, remove or edit entries on the scoreboard.
#[derive(Debug, Clone, Packet)]
pub struct SetScore<'a> {
    /// The type of the action to execute upon the scoreboard with the entries that the packet has.
    /// If `action_type` is `Modify`, all entries will be added to the scoreboard if not yet
    /// present, or modified if already present. If set to `Remove`, all scoreboard entries set will
    /// be removed from the scoreboard.
    pub action_type: ScoreboardAction,
    /// A list of all entries that the client should operate on. When modifying, it will add or
    /// modify all entries, whereas when removing, it will remove all entries.
    pub entries: Vec<ScoreboardEntry<'a>>,
}

impl<'a> Encode for SetScore<'a> {
    fn encode(&self, w: &mut Writer) {
        self.action_type.encode(w);

        w32::from_usize(self.entries.len()).encode(w);

        self.entries
            .iter()
            .for_each(|entry| entry.write(w, &self.action_type));
    }
}

impl<'a> Decode<'a> for SetScore<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let action_type = ScoreboardAction::decode(r)?;

        let len = w32::decode(r)?.to_usize();
        let entries: Vec<_> = (0..len).filter_map(|_| ScoreboardEntry::read(r, &action_type)).collect();

        Some(Self {
            action_type,
            entries
        })
    }
}
