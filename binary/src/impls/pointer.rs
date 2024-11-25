use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;
use crate::{Decode, Encode, Reader, Writer};

impl<T: Encode + ?Sized> Encode for &T {
    fn encode(&self, w: &mut Writer)  {
        (**self).encode(w)
    }
}

impl<T: Encode + ?Sized> Encode for &mut T {
    fn encode(&self, w: &mut Writer)  {
        (**self).encode(w)
    }
}

impl<T: Encode + ?Sized> Encode for Box<T> {
    fn encode(&self, w: &mut Writer)  {
        self.as_ref().encode(w)
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Box<T> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        T::decode(r).map(Box::new)
    }
}

impl<T: Encode + ?Sized> Encode for Rc<T> {
    fn encode(&self, w: &mut Writer)  {
        self.as_ref().encode(w)
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Rc<T> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        T::decode(r).map(Rc::new)
    }
}

impl<T: Encode + ?Sized> Encode for Arc<T> {
    fn encode(&self, w: &mut Writer)  {
        self.as_ref().encode(w)
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Arc<T> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        T::decode(r).map(Arc::new)
    }
}

impl<'a, B> Encode for Cow<'a, B>
where
    B: ToOwned + Encode + ?Sized,
{
    fn encode(&self, w: &mut Writer) {
        self.as_ref().encode(w)
    }
}

impl<'a, 'b, B> Decode<'a> for Cow<'b, B>
where
    B: ToOwned + ?Sized,
    B::Owned: Decode<'a>,
{
    fn decode(r: &mut &'a [u8]) -> Option<Self> {
        B::Owned::decode(r).map(Cow::Owned)
    }
}