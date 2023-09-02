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

// FString type should be reference

pub struct Entity<'a> {
    origin: EntityOrigin,
    tag: Option<Tag>,
    pub id: usize,
    pub flags: u32,
    pub type_idx: u16,
    pub type_name: &'a FString,
    pub esh: ESH,
    enc_size: usize
}

pub struct EntityOpt<'b> {
    pub origin: EntityOrigin,
    pub flags: u32,
    pub type_idx: u16,  
    pub type_name: Option<&'b FString>,
}

impl<'a> Decoder for Entity<'a> {
    type Opt = EntityOpt<'a>;
    fn decode(raw: &Raw, offset: usize, _: usize, opt: Option<Self::Opt>) -> Result<Entity<'a>> {
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
