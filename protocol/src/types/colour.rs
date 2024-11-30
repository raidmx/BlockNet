use bytes::BufMut;
use binary::{Decode, Encode, Reader, w32, Writer};
use derive::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Encode for RGBA {
    fn encode(&self, w: &mut Writer) {
        w.put_u32_le(
            (self.r as u32)
                | ((self.g as u32) << 8)
                | ((self.b as u32) << 16)
                | ((self.a as u32) << 24),
        );
    }
}

impl Decode<'_> for RGBA {
    fn decode(r: &mut Reader) -> Option<Self> {
        let value = u32::decode(r)?;
        Some(Self {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        })
    }
}

#[derive(Debug, Clone)]
pub struct VarRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Encode for VarRGBA {
    fn encode(&self, w: &mut Writer) {
        w32::new(
            (self.r as u32)
                | ((self.g as u32) << 8)
                | ((self.b as u32) << 16)
                | ((self.a as u32) << 24),
        ).encode(w);
    }
}

impl Decode<'_> for VarRGBA {
    fn decode(r: &mut Reader) -> Option<Self> {
        let value = w32::decode(r)?.value();
        Some(Self {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        })
    }
}
