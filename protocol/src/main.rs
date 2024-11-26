use std::io::Read;
use bytes::BytesMut;
use binary::Encode;
use derive::{Decode, Encode};
pub mod nbt;

#[derive(Debug, Encode, Decode)]
#[encoding(type = i16)]
pub enum TestEnum {
    Variant1 = 10,
    Variant2 = -9
}

fn main() {
    let mut w = BytesMut::with_capacity(1500);
    TestEnum::Variant1.encode(&mut w);

    println!("{:?}", w.freeze().bytes());
}