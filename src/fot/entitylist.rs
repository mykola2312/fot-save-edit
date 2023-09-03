use super::decoder::DecoderCtx;
use super::entity::Entity;
use super::esh::ESH;
use super::fstring::FString;
use super::raw::Raw;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::anyhow;
use anyhow::Result;

#[derive(Clone, Copy)]
pub enum EntityEncoding {
    File,
    World,
}

pub struct EntityList {
    encoding: EntityEncoding,
    entity_file_tag: Option<Tag>,
    entity_tag: Option<Tag>,

    types: Vec<FString>,
    entities: Vec<Entity>
}

impl EntityList {
    pub fn get_entity_encoding(&self) -> EntityEncoding {
        self.encoding
    }

    pub fn get_entity_tag(&self) -> &Tag {
        self.entity_tag.as_ref().unwrap()
    }

    pub fn add_new_type(&mut self, type_name: FString) -> usize {
        self.types.push(type_name);
        self.types.len() - 1
    }

    pub fn add_or_get_type(&mut self, type_name: FString) -> usize {
        match self.types.iter().position(|f| f.eq(&type_name)) {
            Some(idx) => idx,
            None => self.add_new_type(type_name)
        }
    }

    pub fn get_type_name(&self, type_idx: usize) -> &FString {
        &self.types[type_idx]
    }
}
