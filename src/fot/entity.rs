use super::raw::Raw;
use super::tag::Tag;
use super::esh::ESH;
use super::decoder::Decoder;
use super::fstring::FString;
use super::stream::{ReadStream, WriteStream};
use anyhow::anyhow;
use anyhow::Result;

pub enum EntityEncoding {
    File,
    World
}

pub trait EntityOwner<'a> {
    fn get_entity_encoding(&self) -> EntityEncoding;
    fn add_new_type(name: FString) -> usize;
    fn add_or_get_type(name: FString) -> usize;
    fn get_type_name(type_idx: usize) -> &'a FString;
}

pub struct Entity {
    pub flags: u32,
    pub type_idx: usize,
    pub esh: ESH,
    enc_size: usize
}

pub struct EntityOpt {
    //pub origin: EntityOrigin,
    pub type_idx: usize
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
