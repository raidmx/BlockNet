use num_derive::{FromPrimitive, ToPrimitive};
use binary::{Array, U8};
use derive::{Decode, Encode};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum Ability {
    Build,
    Mine,
    DoorsAndSwitches,
    OpenContainers,
    AttackPlayers,
    AttackMobs,
    OperatorCommands,
    Teleport,
    Invulnerable,
    Flying,
    MayFly,
    InstantBuild,
    Lightning,
    FlySpeed,
    WalkSpeed,
    Muted,
    WorldBuilder,
    NoClip,
    PrivilegedBuilder,
    Count,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u16)]
pub enum AbilityLayerType {
    CustomCache,
    Base,
    Spectator,
    Commands,
    Editor,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct AbilityLayer {
    layer_type: AbilityLayerType,
    abilities: u32,
    values: u32,
    fly_speed: f32,
    walk_speed: f32,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u16)]
pub struct AbilityData {
    pub entity_unique_id: i64,
    pub player_permissions: u8,
    pub command_permission: u8,
    pub layers: Array<U8, AbilityLayer>,
}
