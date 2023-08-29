use super::decoder::Decoder;
use super::fstring::FString;
use super::raw::Raw;
use super::stream::ReadStream;
use super::tag::Tag;
use anyhow::anyhow;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use deflate::deflate_bytes_zlib;
use inflate::inflate_bytes_zlib;
use std::io::Cursor;

#[derive(Debug)]
pub struct World {
    pub tag: Tag,
    pub uncompressed_size: u32,

    pub data: Raw,
}

impl World {
    const WORLD_TAG_LEN: usize = 11;
    const WORLD_HDR_LEN: usize = 0x13;

    pub fn test(&self) -> Result<()> {
        let sgd_start: usize = 0x4E;
        let mut sgd = ReadStream::new(&self.data, sgd_start);
        let _ = sgd.read::<Tag>(0)?;
        sgd.skip(0x48);

        let n = sgd.read_u32()?;
        dbg!(sgd.offset(), n);
        let mut names: Vec<FString> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            names.push(sgd.read::<FString>(0)?);
        }
        dbg!(names);

        let unk1 = sgd.read_u32()?;
        dbg!(unk1);
        for _ in 0..n {
            let m = sgd.read_u32()?;
            let mut replics: Vec<FString> = Vec::with_capacity(m as usize);
            for _ in 0..m {
                replics.push(sgd.read::<FString>(0)?);
            }
            dbg!(replics);
        }

        Ok(())
    }
}

impl Decoder for World {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self> {
        let tag = Tag::decode(raw, offset, Self::WORLD_TAG_LEN)?;

        let mut rdr = Cursor::new(&raw.mem[offset + Self::WORLD_TAG_LEN..]);
        let uncompressed_size = rdr.read_u32::<LittleEndian>()?;

        let data_start = offset + Self::WORLD_HDR_LEN;
        let data =
            inflate_bytes_zlib(&raw.mem[data_start..data_start + size]).map_err(|e| anyhow!(e))?;
        Ok(World {
            tag,
            uncompressed_size,
            data: Raw {
                offset,
                size,
                mem: data,
            },
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
