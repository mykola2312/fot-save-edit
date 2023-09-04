use super::decoder::Decoder;
use super::fstring::FString;
use super::entitylist::{EntityList, EntityEncoding};
use super::raw::Raw;
use super::sgd::SGD;
use super::ssg::SSG;
use super::stream::ReadStream;
use super::tag::Tag;
use anyhow::anyhow;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use deflate::deflate_bytes_zlib;
use inflate::inflate_bytes_zlib;
use std::io::Cursor;

pub struct World {
    pub tag: Tag,
    pub uncompressed_size: u32,

    pub data: Raw,

    pub mission: FString,
    pub sgd: SGD,
    pub ssg: SSG,

    pub ents: EntityList
}

impl World {
    const WORLD_TAG_LEN: usize = 11;
    const WORLD_HDR_LEN: usize = 0x13;

    pub fn test(&self) -> Result<()> {
        for i in 0..self.ents.entities.len() {
            let ent = &self.ents.entities[i];
            let idx = i+1;

            let type_name = match ent.type_idx {
                0xFFFF => "<NO ESH>",
                _ => self.ents.get_type_name(ent.type_idx).str.as_str()
            };
            println!("idx {} type {}", idx, type_name);
            
            match ent.esh.as_ref() {
                Some(esh) => {
                    for (name, value) in esh.props.iter() {
                        println!("\t{} {}", &name, &value);
                    }
                },
                None => continue
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

        let ents: EntityList = rd.read_opt(0, EntityEncoding::World)?;

        Ok(World {
            tag,
            uncompressed_size,
            data,
            mission,
            sgd,
            ssg,
            ents
        })
    }

    fn encode(&self) -> Result<Raw> {
        let mut hdr = [0u8; 8];
        {
            let mut wdr = Cursor::new(&mut hdr[..]);
            let _ = wdr.write_u32::<LittleEndian>(self.uncompressed_size);
            let _ = wdr.write_u32::<LittleEndian>(self.uncompressed_size);
        }
        let data = deflate_bytes_zlib(&self.data.mem);

        Ok(Raw::join(
            self.data.offset,
            self.data.size,
            &mut [
                self.tag.encode()?,
                Raw {
                    offset: Self::WORLD_TAG_LEN,
                    size: 8,
                    mem: hdr.to_vec(),
                },
                Raw {
                    offset: Self::WORLD_HDR_LEN,
                    size: data.len(),
                    mem: data,
                },
            ],
        ))
    }

    fn get_enc_size(&self) -> usize {
        Self::WORLD_HDR_LEN + self.data.mem.len()
    }
}
