use binary::{v32, w64};
use derive::{Decode, Encode};

/// Sent by the server to apply an effect to the player, for example an effect like poison. It may
/// also be used to modify existing effects, or removing them completely.
#[derive(Debug, Clone, Encode, Decode)]
pub struct MobEffect {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: w64,
    /// The operation of the packet, specifying the result client-side.
    pub operation: MobEffectOperation,
    /// The type of the effect to be added, removed or modified.
    pub effect_type: MobEffectType,
    /// The amplifier of the effect. Take note that the amplifier is not the same as the effect's
    /// level. The level is usually one higher than the amplifier, and the amplifier can be negative
    /// to reverse the behaviour effect.
    pub amplifier: v32,
    /// Specifies if viewers of the entity that gets the effect shows particles around it. If set to
    /// false, no particles are emitted around the entity.
    pub particles: bool,
    /// The duration of the effect in seconds. After the duration has elapsed, the effect will be
    /// removed automatically client-side.
    pub duration: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum MobEffectOperation {
    Add = 1,
    Modify,
    Remove,
}

#[derive(Debug, Default, Clone, Encode, Decode)]
#[encoding(type = w32)]
pub enum MobEffectType {
    #[default]
    Unknown = 0,
    Speed = 1,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneation,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
    Levitation,
    FatalPoison,
    ConduitPower,
    SlowFalling,
}
