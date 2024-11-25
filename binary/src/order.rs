use bytes::{Buf, BufMut};
use crate::{Reader, Writer};

pub trait ByteOrder {
    fn get_u16(r: &mut Reader) -> Option<u16>;
    fn put_u16(w: &mut Writer, v: u16);

    fn get_i16(r: &mut Reader) -> Option<i16>;
    fn put_i16(w: &mut Writer, v: i16);

    fn get_u32(r: &mut Reader) -> Option<u32>;
    fn put_u32(w: &mut Writer, v: u32);

    fn get_i32(r: &mut Reader) -> Option<i32>;
    fn put_i32(w: &mut Writer, v: i32);

    fn get_u64(r: &mut Reader) -> Option<u64>;
    fn put_u64(w: &mut Writer, v: u64);

    fn get_i64(r: &mut Reader) -> Option<i64>;
    fn put_i64(w: &mut Writer, v: i64);

    fn get_f32(r: &mut Reader) -> Option<f32>;
    fn put_f32(w: &mut Writer, v: f32);

    fn get_f64(r: &mut Reader) -> Option<f64>;
    fn put_f64(w: &mut Writer, v: f64);
}

#[derive(Debug, Clone, Copy)]
pub struct LE;

#[derive(Debug, Clone, Copy)]
pub struct BE;

impl ByteOrder for LE {
    fn get_u16(r: &mut Reader) -> Option<u16> {
        if r.remaining() < size_of::<u16>() {
            return None;
        }
        Some(r.get_u16_le())
    }

    fn put_u16(w: &mut Writer, v: u16) {
        w.put_u16_le(v);
    }

    fn get_i16(r: &mut Reader) -> Option<i16> {
        if r.remaining() < size_of::<i16>() {
            return None;
        }
        Some(r.get_i16_le())
    }

    fn put_i16(w: &mut Writer, v: i16) {
        w.put_i16_le(v);
    }

    fn get_u32(r: &mut Reader) -> Option<u32> {
        if r.remaining() < size_of::<u32>() {
            return None;
        }
        Some(r.get_u32_le())
    }

    fn put_u32(w: &mut Writer, v: u32) {
        w.put_u32_le(v);
    }

    fn get_i32(r: &mut Reader) -> Option<i32> {
        if r.remaining() < size_of::<i32>() {
            return None;
        }
        Some(r.get_i32_le())
    }

    fn put_i32(w: &mut Writer, v: i32) {
        w.put_i32_le(v);
    }

    fn get_u64(r: &mut Reader) -> Option<u64> {
        if r.remaining() < size_of::<u64>() {
            return None;
        }
        Some(r.get_u64_le())
    }

    fn put_u64(w: &mut Writer, v: u64) {
        w.put_u64_le(v);
    }

    fn get_i64(r: &mut Reader) -> Option<i64> {
        if r.remaining() < size_of::<i64>() {
            return None;
        }
        Some(r.get_i64_le())
    }

    fn put_i64(w: &mut Writer, v: i64) {
        w.put_i64_le(v);
    }

    fn get_f32(r: &mut Reader) -> Option<f32> {
        if r.remaining() < size_of::<f32>() {
            return None;
        }
        Some(r.get_f32_le())
    }

    fn put_f32(w: &mut Writer, v: f32) {
        w.put_f32_le(v);
    }

    fn get_f64(r: &mut Reader) -> Option<f64> {
        if r.remaining() < size_of::<f64>() {
            return None;
        }
        Some(r.get_f64_le())
    }

    fn put_f64(w: &mut Writer, v: f64) {
        w.put_f64_le(v);
    }
}

impl ByteOrder for BE {
    fn get_u16(r: &mut Reader) -> Option<u16> {
        if r.remaining() < size_of::<u16>() {
            return None;
        }
        Some(r.get_u16())
    }

    fn put_u16(w: &mut Writer, v: u16) {
        w.put_u16(v);
    }

    fn get_i16(r: &mut Reader) -> Option<i16> {
        if r.remaining() < size_of::<i16>() {
            return None;
        }
        Some(r.get_i16())
    }

    fn put_i16(w: &mut Writer, v: i16) {
        w.put_i16(v);
    }

    fn get_u32(r: &mut Reader) -> Option<u32> {
        if r.remaining() < size_of::<u32>() {
            return None;
        }
        Some(r.get_u32())
    }

    fn put_u32(w: &mut Writer, v: u32) {
        w.put_u32(v);
    }

    fn get_i32(r: &mut Reader) -> Option<i32> {
        if r.remaining() < size_of::<i32>() {
            return None;
        }
        Some(r.get_i32())
    }

    fn put_i32(w: &mut Writer, v: i32) {
        w.put_i32(v);
    }

    fn get_u64(r: &mut Reader) -> Option<u64> {
        if r.remaining() < size_of::<u64>() {
            return None;
        }
        Some(r.get_u64())
    }

    fn put_u64(w: &mut Writer, v: u64) {
        w.put_u64(v);
    }

    fn get_i64(r: &mut Reader) -> Option<i64> {
        if r.remaining() < size_of::<i64>() {
            return None;
        }
        Some(r.get_i64())
    }

    fn put_i64(w: &mut Writer, v: i64) {
        w.put_i64(v);
    }

    fn get_f32(r: &mut Reader) -> Option<f32> {
        if r.remaining() < size_of::<f32>() {
            return None;
        }
        Some(r.get_f32())
    }

    fn put_f32(w: &mut Writer, v: f32) {
        w.put_f32(v);
    }

    fn get_f64(r: &mut Reader) -> Option<f64> {
        if r.remaining() < size_of::<f64>() {
            return None;
        }
        Some(r.get_f64())
    }

    fn put_f64(w: &mut Writer, v: f64) {
        w.put_f64(v);
    }
}