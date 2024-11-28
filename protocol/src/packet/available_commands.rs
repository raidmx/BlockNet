use binary::{Decode, Encode, Numeric, Reader, w32, Writer};
use derive::{Decode, Encode, Packet};
use crate::types::command::{CommandEnum, CommandEnumConstraint};

/// Sent by the server to define a list of all commands that the client can use on the server, along
/// with how to use them.
#[derive(Debug, Clone, Packet)]
pub struct AvailableCommands {
    pub enum_values: Vec<String>,
    pub chained_subcommand_values: Vec<String>,
    pub suffixes: Vec<String>,
    pub enums: Vec<CommandEnum>,
    pub chained_subcommands: Vec<ChainedSubcommand>,
    pub commands: Vec<Command>,
    pub dynamic_enums: Vec<DynamicEnum>,
    pub constraints: Vec<CommandEnumConstraint>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub flags: u16,
    pub permission_level: u8,
    pub aliases_offset: u32,
    pub chained_subcommand_offsets: Vec<u16>,
    pub overloads: Vec<CommandOverload>,
}

/// An overload specifies one specific way to use a command.
///
/// Can be compared to operator overloading in languages such as Java or C++. Commands are often
/// given different subcommands by specifying multiple overloads with different signatures and a
/// subcommand name as first parameter. This is not the only use for this however.
#[derive(Debug, Clone, Encode, Decode)]
pub struct CommandOverload {
    /// If true, the command overload uses chained subcommands.
    pub chaining: bool,
    /// À list of parameters specying the usage of the command when this specific overload is
    /// applied.
    pub parameters: Vec<CommandParameter>,
}

/// A single parameter in a command overload. Corresponds to each all possible values accepted at
/// a certain position in the command in this certain overload.
///
/// An example of such a parameter is for instance the choice between `survival`, `creative` and
/// `adventure` mode in the `/gamemode <mode>` command.
#[derive(Debug, Clone, Encode, Decode)]
pub struct CommandParameter {
    pub name: String,
    pub parameter_type: u32, // todo: give this a type
    pub optional: bool,
    pub options: CommandParameterOption,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
#[encoding(type = u8)]
pub enum CommandParameterOption {
    #[default]
    None = 0,
    CollapseEnum,
    HasSemanticConstraint,
    AsChainedCommand,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ChainedSubcommand {
    pub name: String,
    pub values: Vec<ChainedSubcommandValue>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ChainedSubcommandValue {
    pub index: u16,
    pub value: u16,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct DynamicEnum {
    pub type_name: String,
    pub values: String,
}

impl Decode<'_> for AvailableCommands {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let enum_values = Vec::decode(r)?;
        let chained_subcommand_values = Vec::decode(r)?;
        let suffixes = Vec::decode(r)?;

        let enums_len = w32::decode(r)?.to_usize();
        let mut enums = Vec::with_capacity(enums_len);

        for _ in 0..enums_len {
            let enum_type = String::decode(r)?;
            let value_indices_len = w32::decode(r)?.to_usize();

            let mut value_indices = Vec::with_capacity(value_indices_len);

            for _ in 0..value_indices_len {
                value_indices.push(if enum_values.len() < (u8::MAX as usize) {
                    u8::decode(r)? as u32
                } else if enum_values.len() < (u16::MAX as usize) {
                    u16::decode(r)? as u32
                } else {
                    u32::decode(r)?
                });
            }

            enums.push(CommandEnum {
                enum_type,
                value_indices,
            });
        }

        let chained_subcommands = Vec::decode(r)?;
        let commands = Vec::decode(r)?;
        let dynamic_enums = Vec::decode(r)?;
        let constraints = Vec::decode(r)?;

        Some(Self {
            enum_values,
            chained_subcommand_values,
            suffixes,
            enums,
            chained_subcommands,
            commands,
            dynamic_enums,
            constraints,
        })
    }
}

impl Encode for AvailableCommands {
    fn encode(&self, w: &mut Writer) {
        self.enum_values.encode(w);
        self.chained_subcommand_values.encode(w);
        self.suffixes.encode(w);

        w32::new(self.enums.len() as u32).encode(w);

        for v in &self.enums {
            v.enum_type.encode(w);

            w32::new(v.value_indices.len() as u32).encode(w);

            for v in v.value_indices.iter().cloned() {
                if self.enum_values.len() < u8::MAX as usize {
                    (v as u8).encode(w);
                } else if self.enum_values.len() < u16::MAX as usize {
                    (v as u16).encode(w);
                } else {
                    v.encode(w);
                }
            }
        }

        self.chained_subcommands.encode(w);
        self.commands.encode(w);
        self.dynamic_enums.encode(w);
        self.constraints.encode(w);
    }
}
