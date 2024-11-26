use binary::{Decode, Encode, Reader, RefString, VarI32, VarI64, VarU32, Writer, I32, I64, LE, U16};

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
#[derive(Debug, Clone, Copy)]
pub struct NetworkLittleEndian;

/// LittleEndian encoding is used for encoding NBT objects for saving NBT files locally such as player world saves,
/// player data, etc.
#[derive(Debug, Clone, Copy)]
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
        Some(I32::<LE>::decode(r)?.get())
    }

    fn write_int(w: &mut Writer, val: i32) {
        I32::<LE>::new(val).encode(w);
    }

    fn read_long(r: &mut Reader) -> Option<i64> {
        Some(I64::<LE>::decode(r)?.get())
    }

    fn write_long(w: &mut Writer, val: i64) {
        I64::<LE>::new(val).encode(w);
    }

    fn read_str<'a>(r: &mut Reader<'a>) -> Option<&'a str> {
        Some(RefString::<'a, U16<LE>>::decode(r)?.get())
    }

    fn write_str(w: &mut Writer, val: &str) {
        RefString::<U16<LE>>::new(val).encode(w);
    }
}