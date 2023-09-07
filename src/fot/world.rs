use super::decoder::{Decoder, DecoderCtx};
use super::entitylist::{EntityEncoding, EntityList};
use super::fstring::FString;
use super::raw::Raw;
use super::sgd::SGD;
use super::ssg::SSG;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::anyhow;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use deflate::deflate_bytes_zlib;
use inflate::inflate_bytes_zlib;
use std::io::Cursor;

use std::path::Path;

pub struct World {
    pub offset: usize,
    pub size: usize,

    pub tag: Tag,
    pub uncompressed_size: u32,
    //pub data: Raw,

    pub mission: FString,
    pub sgd: SGD,
    pub ssg: SSG,

    pub entlist: EntityList,

    pub unparsed: Vec<u8>
}

impl World {
    const WORLD_TAG_LEN: usize = 11;
    const WORLD_HDR_LEN: usize = 0x13;

    pub fn test(&mut self) -> Result<()> {
        let actor_type = self.entlist.get_type_idx("Actor").unwrap();
        for (id, ent) in &self.entlist {
            if ent.type_idx == actor_type {
                println!("Actor {}", id);
            }
        }

        Ok(())
    }
}

impl Decoder for World {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self> {
        let mut enc = ReadStream::new(raw, offset);

        let tag: Tag = enc.read(Self::WORLD_TAG_LEN)?;
        let uncompressed_size = enc.read_u32()?;
        enc.skip(4);

        let data = {
            let inflated = inflate_bytes_zlib(enc.as_bytes(size)?).map_err(|e| anyhow!(e))?;
            Raw {
                offset,
                size,
                mem: inflated,
            }
        };
        let mut rd = ReadStream::new(&data, 0);

        let mission: FString = rd.read(0)?;
        let sgd: SGD = rd.read(0)?;
        let ssg: SSG = rd.read(0)?;

        let entlist: EntityList = rd.read_opt(0, EntityEncoding::World)?;

        let unparsed = rd.read_bytes(data.mem.len() - rd.offset())?;

        Ok(World {
            offset,
            size,
            tag,
            uncompressed_size,
            //data,
            mission,
            sgd,
            ssg,
            entlist,
            unparsed,
        })
    }

    fn encode(&self) -> Result<Raw> {
        let data = {
            let mut wd = WriteStream::new(self.uncompressed_size as usize);
            
            wd.write(&self.mission)?;
            wd.write(&self.sgd)?;
            wd.write(&self.ssg)?;
            wd.write_opt(&self.entlist, EntityEncoding::World)?;
            wd.write_bytes(&self.unparsed);

            let raw = wd.into_raw(0, 0);
            deflate_bytes_zlib(&raw.mem)
        };

        let mut wd = WriteStream::new(self.get_enc_size());
        wd.write(&self.tag)?;
        wd.write_u32(self.uncompressed_size)?;
        wd.write_u32(self.uncompressed_size)?;
        wd.write_bytes(&data);

        Ok(wd.into_raw(self.offset, self.size))
    }

    fn get_enc_size(&self) -> usize {
        Self::WORLD_HDR_LEN + self.size
    }
}
