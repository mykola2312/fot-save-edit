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
    enc_size: usize,

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

impl DecoderCtx<EntityEncoding> for EntityList {
    fn decode(raw: &Raw, offset: usize, size: usize, ctx: EntityEncoding) -> Result<Self> {
        let mut rd = ReadStream::new(raw, offset);
        let mut ent_list = EntityList {
            encoding: ctx,
            entity_file_tag: None,
            entity_tag: None,
            enc_size: 0,
            types: Vec::new(),
            entities: Vec::new()
        };

        Ok(match ctx {
            EntityEncoding::File => {
                let mut first = true;
                while rd.offset() < size {
                    let tag: Tag = rd.read(0)?;
                    if (first) {
                        ent_list.entity_tag = Some(tag);
                        first = false;
                    }

                    let ent: Entity = rd.read_opt(0, &mut ent_list)?;
                    ent_list.entities.push(ent);
                }
                
                ent_list.enc_size = rd.offset() - offset;
                ent_list
            },

            EntityEncoding::World => {


                ent_list.enc_size = rd.offset() - offset;
                ent_list
            }
        })
    }

    fn encode(&self, ctx: EntityEncoding) -> Result<Raw> {
        todo!();
    }

    fn get_enc_size(&self) -> usize {
        todo!();
    }
}
