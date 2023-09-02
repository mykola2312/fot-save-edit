use super::decoder::DecoderOpt;
use super::entitylist::{EntityEncoding, EntityList};
use super::esh::ESH;
use super::fstring::FString;
use super::raw::Raw;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::anyhow;
use anyhow::Result;

pub struct Entity {
    pub flags: u32,
    pub type_idx: usize,
    pub esh: ESH,
    enc_size: usize,
}

impl DecoderOpt<&EntityList> for Entity {
    fn decode(raw: &Raw, offset: usize, _: usize, opt: &EntityList) -> Result<Self> {
        let mut rd = ReadStream::new(raw, offset);

        todo!();
    }

    fn encode(&self) -> Result<Raw> {
        todo!();
    }

    fn get_enc_size(&self) -> usize {
        todo!();
    }
}
