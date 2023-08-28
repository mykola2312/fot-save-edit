use crate::fot::decoder::Decoder;
use crate::fot::raw::Raw;
use crate::fot::tag::Tag;
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

    fn encode(&self) -> Raw {
        let mut hdr = [0u8; 8];
        {
            let mut wdr = Cursor::new(&mut hdr[..]);
            let _ = wdr.write_u32::<LittleEndian>(self.uncompressed_size);
            let _ = wdr.write_u32::<LittleEndian>(self.uncompressed_size);
        }
        let data = deflate_bytes_zlib(&self.data.mem);

        Raw::join(self.data.offset, self.data.size, &mut [
            self.tag.encode(),
            Raw { offset: Self::WORLD_TAG_LEN, size: 8, mem: hdr.to_vec()},
            Raw { offset: Self::WORLD_HDR_LEN, size: data.len(), mem: data}
        ])
    }

    fn get_enc_len(&self) -> usize {
        Self::WORLD_HDR_LEN + self.data.mem.len()
    }
}