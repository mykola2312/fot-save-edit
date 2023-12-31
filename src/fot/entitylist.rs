use super::decoder::{Decoder, DecoderCtx};
use super::entity::Entity;
use super::ferror::FError as FE;
use super::fstring::{FString, FStringEncoding};
use super::stream::{ReadStream, WriteStream};
use super::tag::{CTag, Tag};
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
pub enum EntityEncoding {
    File,
    World,
}

const DEFAULT_ENTITY_TAG: CTag<'static> = CTag {
    name: "<entity>",
    version: "2",
};

pub struct EntityList {
    encoding: EntityEncoding,
    entity_file_tag: Option<Tag>,
    entity_tag: Option<Tag>,
    unk1: u32,
    enc_size: usize,

    types: Vec<FString>,
    ents: Vec<Entity>,
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

    pub fn get_type_idx(&self, type_name: &str) -> Option<usize> {
        self.types.iter().position(|f| f.eq(type_name))
    }

    pub fn add_or_get_type(&mut self, type_name: FString) -> usize {
        match self.types.iter().position(|f| f.eq(&type_name)) {
            Some(idx) => idx,
            None => self.add_new_type(type_name),
        }
    }

    pub fn get_type_name(&self, type_idx: usize) -> &FString {
        &self.types[type_idx]
    }

    pub fn get_entity(&self, id: usize) -> &Entity {
        &self.ents[id - 1]
    }

    pub fn get_entity_mut(&mut self, id: usize) -> &mut Entity {
        &mut self.ents[id - 1]
    }

    pub fn dump_to_entfile(&self, ent: &Entity, path: &Path) -> Result<(), FE> {
        let esh = match &ent.esh {
            Some(esh) => esh,
            None => return Err(FE::EntityNoESH),
        };

        let tag = DEFAULT_ENTITY_TAG.to_tag();
        let mut type_name = self.get_type_name(ent.type_idx).clone();
        type_name.encoding = FStringEncoding::ANSI;

        let mut wd = WriteStream::new(tag.get_enc_size() + ent.get_enc_size());
        wd.write(&tag)?;
        wd.write(&type_name)?;
        wd.write(esh)?;

        wd.into_raw(0, 0).dump(path)?;
        Ok(())
    }
}

impl DecoderCtx<EntityEncoding, EntityEncoding> for EntityList {
    fn decode<'a>(rd: &mut ReadStream<'a>, ctx: EntityEncoding) -> Result<Self, FE> {
        let offset = rd.offset();
        let mut ent_list = EntityList {
            encoding: ctx,
            entity_file_tag: None,
            entity_tag: None,
            unk1: 0,
            enc_size: 0,
            types: Vec::new(),
            ents: Vec::new(),
        };

        Ok(match ctx {
            EntityEncoding::File => {
                let mut first = true;
                while !rd.is_end() {
                    let tag: Tag = rd.read()?;
                    if first {
                        ent_list.entity_tag = Some(tag);
                        first = false;
                    }

                    let ent: Entity = rd.read_ctx(&mut ent_list)?;
                    ent_list.ents.push(ent);
                }

                ent_list.enc_size = rd.offset() - offset;
                ent_list
            }

            EntityEncoding::World => {
                ent_list.entity_file_tag = Some(rd.read()?);
                let type_count = rd.read_u32()?;
                for _ in 0..type_count {
                    ent_list.types.push(rd.read()?);
                }

                let ent_count = rd.read_u16()?;
                ent_list.unk1 = rd.read_u32()?;
                for _ in 1..ent_count {
                    let ent: Entity = rd.read_ctx(&mut ent_list)?;
                    ent_list.ents.push(ent);
                }

                ent_list.enc_size = rd.offset() - offset;
                ent_list
            }
        })
    }

    fn encode(&self, wd: &mut WriteStream, ctx: EntityEncoding) -> Result<(), FE> {
        match ctx {
            EntityEncoding::File => {
                for ent in self.ents.iter() {
                    if ent.esh.is_none() {
                        continue;
                    }
                    wd.write(self.get_entity_tag())?;
                    wd.write_ctx(ent, &self)?;
                }
            }
            EntityEncoding::World => {
                wd.write(self.entity_file_tag.as_ref().unwrap())?;
                wd.write_u32(self.types.len() as u32)?;
                for type_name in self.types.iter() {
                    wd.write(type_name)?;
                }

                wd.write_u16((self.ents.len() + 1) as u16)?;
                wd.write_u32(self.unk1)?;
                for ent in self.ents.iter() {
                    wd.write_ctx(ent, &self)?;
                }
            }
        }

        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        self.enc_size
    }
}

impl<'a> IntoIterator for &'a EntityList {
    type Item = (usize, &'a Entity);
    type IntoIter = std::iter::Zip<std::ops::RangeFrom<usize>, std::slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        (1..).zip(&self.ents)
    }
}

impl<'a> IntoIterator for &'a mut EntityList {
    type Item = (usize, &'a mut Entity);
    type IntoIter = std::iter::Zip<std::ops::RangeFrom<usize>, std::slice::IterMut<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        (1..).zip(&mut self.ents)
    }
}
