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

pub struct EntityList {}

impl EntityList {
    fn get_entity_encoding(&self) -> EntityEncoding {
        todo!();
    }
    
    fn get_entity_tag(&self) -> &Tag {
        todo!();
    }

    fn add_new_type(&mut self, name: FString) -> usize {
        todo!();
    }

    fn add_or_get_type(&mut self, name: FString) -> usize {
        todo!();
    }

    fn get_type_name(&self, type_idx: usize) -> &FString {
        todo!();
    }
}

pub struct Entity {
    pub flags: u32,
    pub type_idx: usize,
    pub esh: ESH,
    enc_size: usize
}

impl Decoder for Entity {
    type Opt<'o> = &'o EntityList;
    fn decode<'o>(raw: &Raw, offset: usize, _: usize, opt: Option<Self::Opt<'o>>) -> Result<Self> {
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
