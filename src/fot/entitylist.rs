use super::decoder::DecoderCtx;
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
    pub fn get_entity_encoding(&self) -> EntityEncoding {
        todo!();
    }

    pub fn get_entity_tag(&self) -> &Tag {
        todo!();
    }

    pub fn add_new_type(&mut self, type_name: FString) -> usize {
        todo!();
    }

    pub fn add_or_get_type(&mut self, type_name: FString) -> usize {
        todo!();
    }

    pub fn get_type_name(&self, type_idx: usize) -> &FString {
        todo!();
    }
}
