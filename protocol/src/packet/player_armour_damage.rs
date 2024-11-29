use binary::{Decode, Encode, Reader, Writer};
use derive::Packet;

/// Sent by the server to damage the armour of a player. It is a very efficient packet, but
/// generally it's much easier to just send a slot update for the damaged armour.
#[derive(Debug, Clone, Default, Packet)]
pub struct PlayerArmourDamage {
    /// A bitset of 4 bits that indicate which pieces of armour need to have damage dealt to them.
    /// The first bit, when toggled, is for a helmet, the second for the chestplate, the third for
    /// the leggings and the fourth for boots.
    pub bitset: u8,
    /// The amount of damage that should be dealt to the helmet.
    pub helmet_damage: i32,
    /// The amount of damage that should be dealt to the chestplate.
    pub chestplate_damage: i32,
    /// The amount of damage that should be dealt to the leggings.
    pub leggings_damage: i32,
    /// The amount of damage that should be dealt to the boots.
    pub boots_damage: i32,
}

impl Encode for PlayerArmourDamage {
    fn encode(&self, w: &mut Writer) {
        self.bitset.encode(w);

        if self.bitset & 0x01 != 0 {
            self.helmet_damage.encode(w);
        }
        if self.bitset & 0x02 != 0 {
            self.chestplate_damage.encode(w);
        }
        if self.bitset & 0x04 != 0 {
            self.leggings_damage.encode(w);
        }
        if self.bitset & 0x08 != 0 {
            self.boots_damage.encode(w);
        }
    }
}

impl Decode<'_> for PlayerArmourDamage {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut pk = Self {
            bitset: u8::decode(r)?,
            ..Default::default()
        };

        if pk.bitset & 0x01 != 0 {
            pk.helmet_damage = i32::decode(r)?;
        }
        if pk.bitset & 0x02 != 0 {
            pk.chestplate_damage = i32::decode(r)?;
        }
        if pk.bitset & 0x03 != 0 {
            pk.leggings_damage = i32::decode(r)?;
        }
        if pk.bitset & 0x04 != 0 {
            pk.boots_damage = i32::decode(r)?;
        }

        Some(pk)
    }
}
