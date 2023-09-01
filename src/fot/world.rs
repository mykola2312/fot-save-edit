use super::decoder::Decoder;
use super::esh::ESH;
use super::fstring::FString;
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

#[derive(Debug)]
pub struct World {
    pub tag: Tag,
    pub uncompressed_size: u32,

    pub data: Raw,

    pub mission: FString,
    pub sgd: SGD,
    pub ssg: SSG,
}

impl World {
    const WORLD_TAG_LEN: usize = 11;
    const WORLD_HDR_LEN: usize = 0x13;

    pub fn test(&self) -> Result<()> {
        let mut rd = ReadStream::new(&self.data, 0x1038);
        let _: Tag = rd.read(0)?;

        let n = rd.read_u32()? as usize;
        let mut types: Vec<FString> = Vec::with_capacity(n);
        for i in 0..n {
            let ent_type: FString = rd.read(0)?;
            println!("{}\t{}", i, &ent_type);
            types.push(ent_type);
        }

        let esh_count = rd.read_u16()?;
        let unk1 = rd.read_u32()?;
        dbg!(esh_count);
        for i in 1..esh_count {
            let unk2 = rd.read_u32()?;
            let type_idx = rd.read_u16()?;
            if type_idx == 0xFFFF {
                continue;
            }

            let name = &types[type_idx as usize];
            println!(
                "offset {:x} idx {} unk1 {} name {}",
                rd.offset(),
                i,
                unk2,
                name
            );
            let _: ESH = rd.read(0)?;
        }

        Ok(())
    }
}

impl Decoder for World {
    type Opt = ();
    fn decode(raw: &Raw, offset: usize, size: usize, _: Option<()>) -> Result<Self> {
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

        Ok(World {
            tag,
            uncompressed_size,
            data,
            mission,
            sgd,
            ssg,
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
