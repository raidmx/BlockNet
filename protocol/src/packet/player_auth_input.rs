use crate::types::{Vec2, Vec3};
use num_derive::{FromPrimitive, ToPrimitive};
use binary::{w64, Decode, Encode, Reader, Writer};
use derive::{Decode, Encode, Packet};
use crate::types::inventory::UseItemTransactionData;
use crate::types::item_stack::ItemStackRequestEntry;
use crate::types::player::{InputMode, InteractionModel, PlayerBlockAction};

#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
#[encoding(type = w32)]
pub enum PlayMode {
    #[default]
    Normal,
    Teaser,
    Screen,
    Viewer,
    Reality,
    Placement,
    LivingRoom,
    ExitLevel,
    ExitLevelLivingRoom,
    NumModes,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum InputFlag {
    Ascend,
    Descend,
    NorthJump,
    JumpDown,
    SprintDown,
    ChangeHeight,
    Jumping,
    AutoJumpingInWater,
    Sneaking,
    SneakDown,
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    WantUp,
    WantDown,
    WantDownSlow,
    WantUpSlow,
    Sprinting,
    AscendBlock,
    DescendBlock,
    SneakToggleDown,
    PersistSneak,
    StartSprinting,
    StopSprinting,
    StartSneaking,
    StopSneaking,
    StartSwimming,
    StopSwimming,
    StartJumping,
    StartGliding,
    StopGliding,
    PerformItemInteraction,
    PerformBlockActions,
    PerformItemStackRequest,
    HandledTeleport,
    Emoting,
    StartFlying,
    StopFlying,
    ClientAckServerData,
}

impl InputFlag {
    pub fn flag(&self) -> u64 {
        1 << (*self as u64)
    }
}

/// Sent by the client to allow for server authoritative movement. It is used to synchronise the
/// player input with the position server-side. The client sends this packet when the server
/// authoritative movement mode field in the StartGame packet is set to true. Instead of the
/// MovePlayer packet, the client will send this packet once every tick.
#[derive(Debug, Clone, Default, Packet)]
pub struct PlayerAuthInput<'a> {
    /// The pitch the player reports it has.
    pub pitch: f32,
    /// The yaw the player reports it has.
    pub yaw: f32,
    /// The position that the player reports it has.
    pub position: Vec3,
    /// A Vec2 that specifies the direction in which the player moved, as a combination of X/Z
    /// values which are created using the WASD/controller stick state.
    pub move_vector: Vec2,
    /// The horizontal rotation of the head that the player reports it has.
    pub head_yaw: f32,
    /// A combination of bit flags that together specify the way the player moved last tick.
    pub input_data: w64,
    /// Specifies the way that the client inputs data to the screen.
    pub input_mode: InputMode,
    /// Specifies the way that the player is playing. The values it holds, which are rather random,
    /// may be found above.
    pub play_mode: PlayMode,
    /// The interaction model the player is using.
    pub interaction_model: InteractionModel,
    /// The direction in which the player is gazing, when the `play_mode` is reality. In other
    /// words, when the player is playing in virtual reality.
    pub gaze_direction: Vec3,
    /// The server tick at which the packet was sent. It is used in relation to the
    /// CorrectPlayerMovePrediction packet.
    pub tick: w64,
    /// The delta between the old and the new position. There isn't any practical use for this field
    /// as it can be calculated by the server itself.
    pub delta: Vec3,
    /// The transaction data if the `input_data` includes an item interaction.
    pub item_interaction_data: UseItemTransactionData<'a>,
    /// Sent by the client to change an item in their inventory.
    pub item_stack_request: ItemStackRequestEntry<'a>,
    /// A list of block actions that the client has interacted with.
    pub block_actions: Vec<PlayerBlockAction>,
    /// The direction in which the player moved, as a combination of X/Z values which are created
    /// using an analogue input.
    pub analogue_move_vector: Vec2,
}

impl<'a> Encode for PlayerAuthInput<'a> {
    fn encode(&self, w: &mut Writer) {
        self.pitch.encode(w);
        self.yaw.encode(w);
        self.position.encode(w);
        self.move_vector.encode(w);
        self.head_yaw.encode(w);
        self.input_data.encode(w);
        self.input_mode.encode(w);
        self.play_mode.encode(w);
        self.interaction_model.encode(w);

        if self.play_mode == PlayMode::Reality {
            self.gaze_direction.encode(w);
        }

        self.tick.encode(w);
        self.delta.encode(w);

        if *self.input_data & InputFlag::PerformItemInteraction.flag() != 0 {
            self.item_interaction_data.encode(w);
        }
        if *self.input_data & InputFlag::PerformItemStackRequest.flag() != 0 {
            self.item_stack_request.encode(w);
        }
        if *self.input_data & InputFlag::PerformBlockActions.flag() != 0 {
            self.block_actions.encode(w);
        }

        self.analogue_move_vector.encode(w);
    }
}

impl<'a> Decode<'a> for PlayerAuthInput<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let mut pk = Self {
            pitch: f32::decode(r)?,
            yaw: f32::decode(r)?,
            position: Vec3::decode(r)?,
            move_vector: Vec2::decode(r)?,
            head_yaw: f32::decode(r)?,
            input_data: w64::decode(r)?,
            input_mode: InputMode::decode(r)?,
            play_mode: PlayMode::decode(r)?,
            interaction_model: InteractionModel::decode(r)?,
            ..Default::default()
        };

        if pk.play_mode == PlayMode::Reality {
            pk.gaze_direction = Vec3::decode(r)?;
        }
        if *pk.input_data & InputFlag::PerformItemInteraction.flag() != 0 {
            pk.item_interaction_data = UseItemTransactionData::decode(r)?;
        }
        if *pk.input_data & InputFlag::PerformItemStackRequest.flag() != 0 {
            pk.item_stack_request = ItemStackRequestEntry::decode(r)?;
        }
        if *pk.input_data & InputFlag::PerformBlockActions.flag() != 0 {
            pk.block_actions = Vec::decode(r)?;
        }

        pk.analogue_move_vector = Vec2::decode(r)?;
        Some(pk)
    }
}
