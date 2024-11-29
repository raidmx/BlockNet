use binary::{w32, Encode, Decode, Writer, Reader};
use derive::Packet;
use crate::types::command::{CommandOrigin, CommandOutputMessage, CommandOutputType};

/// Sent by the server to the client to send text as output of a command. Most servers do not use
/// this packet and instead simply send Text packets, but there is reason to send it. If the origin
/// of a CommandRequest packet is not the player itself, but, for example, a WS server, sending a
/// Text packet will not do what is expected: The message should go to the WS server, not to the
/// client's chat. The CommandOutput packet will make sure the messages are relayed to the correct
/// origin of the command request.
#[derive(Debug, Clone, Default, Packet)]
pub struct CommandOutput<'a> {
    /// The data specifying the origin of the command. In other words, the source that the command
    /// request was from, such as the player itself or a WS server. The client forwards the messages
    /// in this packet to the right origin, depending on what is sent here.
    pub command_origin: CommandOrigin,
    /// The type of output that is sent. The vanilla game usually sends all output here.
    pub output_type: CommandOutputType,
    /// The amount of times that a command was executed successfully as a result of the command that
    /// was requested. For servers, this is usually a rather meaningless fields, but for vanilla,
    /// this is applicable for commands created with functions.
    pub success_count: w32,
    /// A list of all output messages that should be sent to the player. Whether they are shown or
    /// not, depends on the type of the messages.
    pub output_messages: Vec<CommandOutputMessage>,
    /// The purpose of this field is currently unknown.
    pub data_set: &'a str,
}

impl<'a> Encode for CommandOutput<'a> {
    fn encode(&self, w: &mut Writer) {
        self.command_origin.encode(w);
        self.output_type.encode(w);
        self.success_count.encode(w);
        self.output_messages.encode(w);

        if self.output_type == CommandOutputType::DataSet {
            self.data_set.encode(w);
        }
    }
}

impl<'a> Decode<'a> for CommandOutput<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let mut pk = Self {
            command_origin: CommandOrigin::decode(r)?,
            output_type: CommandOutputType::decode(r)?,
            success_count: w32::decode(r)?,
            output_messages: Vec::decode(r)?,
            ..Default::default()
        };

        if pk.output_type == CommandOutputType::DataSet {
            pk.data_set = <&'a str>::decode(r)?;
        }

        Some(pk)
    }
}
