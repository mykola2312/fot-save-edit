use super::decoder::DecoderCtx;
use super::entitylist::{EntityEncoding, EntityList};
use super::fstring::FString;
use super::raw::Raw;
use super::sgd::SGD;
use super::ssg::SSG;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::anyhow;
use anyhow::Result;
use deflate::deflate_bytes_zlib;
use inflate::inflate_bytes_zlib;

use super::esh::{ESHValue, ESH};
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

    pub unparsed: Vec<u8>,
}

impl World {
    const WORLD_HDR_LEN: usize = 0x13;

    pub fn test(&mut self) -> Result<()> {
        //let actor_type = self.entlist.get_type_idx("Actor").unwrap();
        let ent = self.entlist.get_entity(2122);
        let esh = ent.esh.as_ref().unwrap();
        for (name, value) in &esh.props {
            println!("{} {}", name, value);
        }
        //self.entlist.dump_to_entfile(ent, Path::new("D:\\actor.ent"))?;

        println!("");
        if let ESHValue::Binary(attributes) = &esh.props["Attributes"] {
            let mut rd = ReadStream::new(&attributes, 0);

            let size = rd.read_u32()?;
            let attrs_esh: ESH = rd.read()?;
            for (name, value) in &attrs_esh.props {
                println!("{} {}", name, value);
            }
        }

        println!("");
        if let ESHValue::Binary(attributes) = &esh.props["Modifiers"] {
            let mut rd = ReadStream::new(&attributes, 0);

            let size = rd.read_u32()?;
            let attrs_esh: ESH = rd.read()?;
            for (name, value) in &attrs_esh.props {
                println!("{} {}", name, value);
            }
        }

        Ok(())
    }
}

pub type WorldOffsetSize = (usize, usize);
impl DecoderCtx<WorldOffsetSize, ()> for World {
    fn decode<'a>(enc: &mut ReadStream<'a>, ctx: WorldOffsetSize) -> Result<Self> {
        let offset = ctx.0;
        let size = ctx.1;

        let tag: Tag = enc.read()?;
        let uncompressed_size = enc.read_u32()?;
        enc.skip(4);

        let data = inflate_bytes_zlib(enc.as_bytes(size)?).map_err(|e| anyhow!(e))?;
        let mut rd = ReadStream::new(&data, 0);

        let mission: FString = rd.read()?;
        let sgd: SGD = rd.read()?;
        let ssg: SSG = rd.read()?;

        let entlist: EntityList = rd.read_ctx(0, EntityEncoding::World)?;

        let unparsed = rd.read_bytes(data.len() - rd.offset())?;

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

    fn encode(&self, wd: &mut WriteStream, _: ()) -> Result<()> {
        let data = {
            let mut wd = WriteStream::new(self.uncompressed_size as usize);

            wd.write(&self.mission)?;
            wd.write(&self.sgd)?;
            wd.write(&self.ssg)?;
            wd.write_ctx(&self.entlist, EntityEncoding::World)?;
            wd.write_bytes(&self.unparsed);

            let raw = wd.into_raw(0, 0);
            deflate_bytes_zlib(&raw.mem)
        };

        wd.write(&self.tag)?;
        wd.write_u32(self.uncompressed_size)?;
        wd.write_u32(self.uncompressed_size)?;
        wd.write_bytes(&data);

        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        Self::WORLD_HDR_LEN + self.size
    }
}
