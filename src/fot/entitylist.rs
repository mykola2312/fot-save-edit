use super::decoder::DecoderOpt;
use super::entity::Entity;
use super::esh::ESH;
use super::fstring::FString;
use super::raw::Raw;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::anyhow;
use anyhow::Result;

pub enum EntityEncoding {
    File,
    World,
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
