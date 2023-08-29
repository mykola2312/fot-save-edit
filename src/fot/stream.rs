use super::decoder::Decoder;
use super::raw::Raw;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub struct ReadStream<'a> {
    raw: &'a Raw,
    rdr: Cursor<&'a [u8]>,
}

impl<'a> ReadStream<'a> {
    fn new(raw: &Raw, offset: usize) -> ReadStream {
        ReadStream {
            raw: raw,
            rdr: Cursor::new(&raw.mem[offset..]),
        }
    }

    fn offset(&self) -> usize {
        self.rdr.position() as usize
    }

    fn skip(&mut self, size: usize) {
        self.rdr.set_position(self.rdr.position() + size as u64);
    }

    fn read<T: Decoder>(&mut self, size: usize) -> Result<T> {
        let val = T::decode(&self.raw, self.offset(), size)?;
        self.skip(val.get_enc_size());
        Ok(val)
    }

    fn read_u16(&mut self) -> Result<u16> {
        Ok(self.rdr.read_u16::<LittleEndian>()?)
    }

    fn read_u32(&mut self) -> Result<u32> {
        Ok(self.rdr.read_u32::<LittleEndian>()?)
    }
}
