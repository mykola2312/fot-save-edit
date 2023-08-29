use crate::fot::fstring::FStringEncoding;

use super::decoder::Decoder;
use super::raw::Raw;
use super::tag::Tag;
use super::fstring::FString;
use anyhow::anyhow;
use anyhow::Result;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use inflate::inflate_bytes_zlib;
use deflate::deflate_bytes_zlib;

#[derive(Debug)]
pub struct World {
    pub tag: Tag,
    pub uncompressed_size: u32,

    pub data: Raw
}

impl World {
    const WORLD_TAG_LEN: usize = 11;
    const WORLD_HDR_LEN: usize = 0x13;

    pub fn test(&self) -> Result<()> {
        let sgd_start: usize = 0x4E;
        let mut cur = sgd_start;
        cur += Tag::decode(&self.data, sgd_start, 0)?.get_enc_size();
        cur += 0x48;
        
        let mut rdr = Cursor::new(&self.data.mem[..]);
        rdr.set_position(cur as u64);
        let N = rdr.read_u32::<LittleEndian>()?;
        
        let mut names: Vec<FString> = Vec::new();
        cur += 4;
        for _ in 0..N {
            let name = FString::decode(&self.data, cur, 0)?;
            cur += name.get_enc_size();
            names.push(name);
        }
        //dbg!(names);

        let unk1 = rdr.read_u32::<LittleEndian>()?;
        cur += 4;
        for _ in 0..N {
            rdr.set_position(cur as u64);
            let M = rdr.read_u32::<LittleEndian>()?;
            cur += 4;
            dbg!(M, cur);
            let mut strings: Vec<FString> = Vec::new();
            for _ in 0..M {
                let str = FString::decode(&self.data, cur, 0)?;
                cur += str.get_enc_size();
                strings.push(str);
            }
            dbg!(strings);
        }

        Ok(())
    }
}

impl Decoder for World {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self> {
        let tag = Tag::decode(raw, offset, Self::WORLD_TAG_LEN)?;
        
        let mut rdr = Cursor::new(&raw.mem[offset+Self::WORLD_TAG_LEN..]);
        let uncompressed_size = rdr.read_u32::<LittleEndian>()?;

        let data_start = offset + Self::WORLD_HDR_LEN;
        let data = inflate_bytes_zlib(&raw.mem[data_start..data_start+size])
            .map_err(|e| anyhow!(e))?;
        Ok(World { tag, uncompressed_size, data: Raw { offset, size, mem: data } })
    }

    fn encode(&self) -> Result<Raw> {
        let mut hdr = [0u8; 8];
        {
            let mut wdr = Cursor::new(&mut hdr[..]);
            let _ = wdr.write_u32::<LittleEndian>(self.uncompressed_size);
            let _ = wdr.write_u32::<LittleEndian>(self.uncompressed_size);
        }
        let data = deflate_bytes_zlib(&self.data.mem);

        Ok(Raw::join(self.data.offset, self.data.size, &mut [
            self.tag.encode()?,
            Raw { offset: Self::WORLD_TAG_LEN, size: 8, mem: hdr.to_vec()},
            Raw { offset: Self::WORLD_HDR_LEN, size: data.len(), mem: data}
        ]))
    }

    fn get_enc_size(&self) -> usize {
        Self::WORLD_HDR_LEN + self.data.mem.len()
    }
}