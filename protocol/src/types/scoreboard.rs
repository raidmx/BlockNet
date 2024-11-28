use num_derive::{FromPrimitive, ToPrimitive};
use binary::{Decode, Encode, Reader, v64, Writer};
use derive::{Decode, Encode};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ScoreboardSortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ScoreboardSlot {
    List,
    Sidebar,
    BelowName,
}

#[derive(Clone, Debug, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = u8)]
pub enum ScoreboardAction {
    Modify,
    Remove,
}

#[repr(u8)]
#[derive(Default, Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum ScoreboardIdentity<'a> {
    #[default]
    None,
    Player(v64),
    Entity(v64),
    FakePlayer(&'a str),
}

#[derive(Debug, Clone)]
pub struct ScoreboardEntry<'a> {
    pub entry_id: v64,
    pub objective_name: &'a str,
    pub score: i32,
    pub identity_type: ScoreboardIdentity<'a>,
}

impl<'a> ScoreboardEntry<'a> {
    pub fn write(&self, w: &mut Writer, action: ScoreboardAction) {
        self.entry_id.encode(w);
        self.objective_name.encode(w);
        self.score.encode(w);

        if let ScoreboardAction::Modify = action {
            self.identity_type.encode(w);
        }
    }

    pub fn read(r: &mut &'a [u8], action: ScoreboardAction) -> Option<Self> {
        let entry_id = v64::decode(r)?;
        let objective_name = <&'a str>::decode(r)?;
        let score = i32::decode(r)?;
        let mut identity_type = ScoreboardIdentity::None;

        if let ScoreboardAction::Modify = action {
            identity_type = ScoreboardIdentity::decode(r)?;
        }

        Some(Self {
            entry_id,
            objective_name,
            score,
            identity_type
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = u8)]
pub enum ScoreboardIdentityAction {
    Register,
    Clear,
}

#[derive(Debug, Clone)]
pub struct ScoreboardIdentityEntry {
    pub entry_id: v64,
    pub entity_unique_id: v64,
}

impl ScoreboardIdentityEntry {
    pub fn write(&self, w: &mut Writer, action: ScoreboardIdentityAction) {
        self.entry_id.encode(w);

        if let ScoreboardIdentityAction::Register = action {
            self.entity_unique_id.encode(w);
        }
    }

    pub fn read(r: &mut Reader, action: ScoreboardIdentityAction) -> Option<Self> {
        let entry_id = v64::decode(r)?;
        let mut entity_unique_id = v64::default();

        if let ScoreboardIdentityAction::Register = action {
            entity_unique_id = v64::decode(r)?;
        }
        Some(Self {
            entry_id,
            entity_unique_id
        })
    }
}