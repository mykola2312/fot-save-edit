use super::raw::Raw;
use super::tag::Tag;
use super::esh::ESH;
use super::decoder::Decoder;
use super::fstring::FString;
use super::stream::{ReadStream, WriteStream};
use anyhow::anyhow;
use anyhow::Result;

pub enum EntityOrigin {
    File,
    World
}

pub struct Entity {
    origin: EntityOrigin,
    tag: Option<Tag>,
    pub id: usize,
    pub flags: u32,
    pub type_idx: u16,
    pub type_name: FString,
    pub esh: ESH,
    enc_size: usize
}

pub struct EntityOpt {
    pub origin: EntityOrigin,
    pub flags: u32,
    pub type_idx: u16,
    pub type_name: Option<FString>,
}

impl Decoder for Entity {
    type Opt = EntityOpt;
    fn decode(raw: &Raw, offset: usize, _: usize, opt: Option<Self::Opt>) -> Result<Self> {
        let rd = ReadStream::new(raw, offset);
        let opt = match opt {
            Some(opt) => opt,
            None => return Err(anyhow!("no EntityOpt was provided!"))
        };

        
        todo!();
    }

    fn encode(&self) -> Result<Raw> {
        todo!();
    }

    fn get_enc_size(&self) -> usize {
        todo!();
    }
}
