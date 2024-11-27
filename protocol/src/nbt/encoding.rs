use binary::{Decode, Encode, Reader, RefString, VarI32, VarI64, VarU32, Writer};

/// Encoding is the trait implemented for the various types of NBT Encoding supported 
/// by the NBT Library
pub trait Encoding {
    fn read_int(r: &mut Reader) -> Option<i32>;
    fn write_int(w: &mut Writer, val: i32);

    fn read_long(r: &mut Reader) -> Option<i64>;
    fn write_long(w: &mut Writer, val: i64);

    fn read_str<'a>(r: &mut Reader<'a>) -> Option<&'a str>;
    fn write_str(w: &mut Writer, val: &str);
}

/// NetworkLittleEndian encoding is used for encoding NBT objects over the network and the wire. It encodes
/// the integers in variable length encoding format which optimizes bandwidth.
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct NetworkLittleEndian;

/// LittleEndian encoding is used for encoding NBT objects for saving NBT files locally such as player world saves,
/// player data, etc.
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct LittleEndian;

impl Encoding for NetworkLittleEndian {
    fn read_int(r: &mut Reader) -> Option<i32> {
        Some(VarI32::decode(r)?.get())
    }

    fn write_int(w: &mut Writer, val: i32) {
        VarI32::new(val).encode(w);
    }

    fn read_long(r: &mut Reader) -> Option<i64> {
        Some(VarI64::decode(r)?.get())
    }

    fn write_long(w: &mut Writer, val: i64) {
        VarI64::new(val).encode(w);
    }

    fn read_str<'a>(r: &mut Reader<'a>) -> Option<&'a str> {
        Some(RefString::<'a, VarU32>::decode(r)?.get())
    }

    fn write_str(w: &mut Writer, val: &str) {
        RefString::<VarU32>::new(val).encode(w);
    }
}

impl Encoding for LittleEndian {
    fn read_int(r: &mut Reader) -> Option<i32> {
        Some(i32::decode(r)?)
    }

    fn write_int(w: &mut Writer, val: i32) {
        val.encode(w);
    }

    fn read_long(r: &mut Reader) -> Option<i64> {
        Some(i64::decode(r)?)
    }

    fn write_long(w: &mut Writer, val: i64) {
        val.encode(w);
    }

    fn read_str<'a>(r: &mut Reader<'a>) -> Option<&'a str> {
        Some(RefString::<'a, u16>::decode(r)?.get())
    }

    fn write_str(w: &mut Writer, val: &str) {
        RefString::<u16>::new(val).encode(w);
    }
}