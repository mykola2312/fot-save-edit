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
    pub fn new(raw: &Raw, start: usize) -> ReadStream {
        let mut rdr = Cursor::new(&raw.mem[..]);
        rdr.set_position(start as u64);
        ReadStream { raw: raw, rdr: rdr }
    }

    pub fn offset(&self) -> usize {
        self.rdr.position() as usize
    }

    pub fn skip(&mut self, size: usize) {
        self.rdr.set_position(self.rdr.position() + size as u64);
    }

    pub fn read<T: Decoder>(&mut self, size: usize) -> Result<T> {
        let val = T::decode(&self.raw, self.offset(), size)?;
        self.skip(val.get_enc_size());
        Ok(val)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(self.rdr.read_u16::<LittleEndian>()?)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(self.rdr.read_u32::<LittleEndian>()?)
    }
}

pub struct WriteStream {
    buf: Cursor<Vec<u8>>,
}

impl WriteStream {
    pub fn new(capacity: usize) -> WriteStream {
        WriteStream {
            buf: Cursor::new(Vec::with_capacity(capacity)),
        }
    }

    pub fn into_raw(self, offset: usize, size: usize) -> Raw {
        let buf_size = self.buf.get_ref().len();
        Raw {
            offset: offset,
            size: if size == 0 { buf_size } else { size },
            mem: self.buf.into_inner(),
        }
    }

    pub fn offset(&self) -> usize {
        self.buf.position() as usize
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.get_mut().extend(bytes.iter());
    }

    pub fn write<T: Decoder>(&mut self, val: &T) -> Result<()> {
        let mut raw = val.encode()?;
        self.buf.get_mut().append(&mut raw.mem);
        Ok(())
    }

    pub fn write_u16(&mut self, val: u16) -> Result<()> {
        Ok(self.buf.write_u16::<LittleEndian>(val)?)
    }

    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        Ok(self.buf.write_u32::<LittleEndian>(val)?)
    }
}
