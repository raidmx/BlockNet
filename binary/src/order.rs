use crate::{Reader, Writer};

pub trait ByteOrder {
    fn put_u16(w: &mut Writer, v: u16);
    fn get_u16(r: &mut Reader) -> Option<u16>;
    fn put_i16(w: &mut Writer, v: i16);
    fn get_i16(r: &mut Reader) -> Option<i16>;
    fn put_u24(w: &mut Writer, v: u32);
    fn get_u24(r: &mut Reader) -> Option<u32>;
    fn put_u32(w: &mut Writer, v: u32);
    fn get_u32(r: &mut Reader) -> Option<u32>;
    fn put_i32(w: &mut Writer, v: i32);
    fn get_i32(r: &mut Reader) -> Option<i32>;
    fn put_u64(w: &mut Writer, v: u64);
    fn get_u64(r: &mut Reader) -> Option<u64>;
    fn put_i64(w: &mut Writer, v: i64);
    fn get_i64(r: &mut Reader) -> Option<i64>;
    fn put_f32(w: &mut Writer, v: f32);
    fn get_f32(r: &mut Reader) -> Option<f32>;
    fn put_f64(w: &mut Writer, v: f64);
    fn get_f64(r: &mut Reader) -> Option<f64>;
}