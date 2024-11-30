use crate::nbt::Compound;

#[derive(Debug, Clone)]
pub struct BlockState<'a> {
    pub name: &'a str,
    pub properties: Compound<'a>,
    pub version: i32
}