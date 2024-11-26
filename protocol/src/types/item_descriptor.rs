use std::fmt::Debug;
use binary::{Decode, Encode, Reader, VarI32, Writer};
use derive::{Decode, Encode};

#[derive(Debug, Clone)]
pub struct DefaultItemDescriptor {
    pub network_id: i16,
    pub metadata: i16
}

impl Encode for DefaultItemDescriptor {
    fn encode(&self, w: &mut Writer) {
        self.network_id.encode(w);

        if self.network_id != 0 {
            self.metadata.encode(w);
        }
    }
}

impl Decode<'_> for DefaultItemDescriptor {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let network_id = i16::decode(r)?;
        let mut metadata = 0;

        if network_id != 0 {
            metadata = i16::decode(r)?;
        }

        Some(Self {
            network_id,
            metadata
        })
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MoLangItemDescriptor {
    pub expression: String,
    pub version: u8
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ItemTagItemDescriptor {
    pub tag: String
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct DeferredItemDescriptor {
    pub name: String,
    pub metadata: i16
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ComplexAliasItemDescriptor {
    pub name: String
}

#[derive(Debug, Clone, Default, Encode, Decode)]
#[encoding(type = u8)]
pub enum ItemDescriptor {
    #[default]
    InvalidDescriptor,
    DefaultDescriptor(DefaultItemDescriptor),
    MoLangDescriptor(MoLangItemDescriptor),
    ItemTagDescriptor(ItemTagItemDescriptor),
    DeferredDescriptor(DeferredItemDescriptor),
    ComplexAliasDescriptor(ComplexAliasItemDescriptor),
}

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct ItemDescriptorCount {
    pub item_descriptor: ItemDescriptor,
    pub count: VarI32,
}