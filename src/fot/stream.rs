use super::decoder::{Decoder, DecoderOpt};
use super::raw::Raw;
use anyhow::anyhow;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub struct ReadStream<'a> {
    raw: &'a Raw,
    rdr: Cursor<&'a [u8]>,
}

impl<'a> ReadStream<'a> {
    pub fn new(raw: &'a Raw, offset: usize) -> ReadStream<'a> {
        let mut rdr = Cursor::new(&raw.mem[..]);
        rdr.set_position(offset as u64);
        ReadStream { raw: raw, rdr: rdr }
    }

    pub fn offset(&self) -> usize {
        self.rdr.position() as usize
    }

    pub fn skip(&mut self, size: usize) {
        self.rdr.set_position(self.rdr.position() + size as u64);
    }

    pub fn as_bytes(&mut self, size: usize) -> Result<&[u8]> {
        if self.offset() + size > self.raw.mem.len() {
            dbg!(self.offset(), size, self.raw.mem.len());
            Err(anyhow!("as_bytes/read_bytes size is bigger than buffer"))
        } else {
            let buf = &self.raw.mem[self.offset()..self.offset() + size];
            self.skip(size);
            Ok(buf)
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        Ok(self.as_bytes(size)?.to_vec())
    }

    // "size" is not required to be actual size, it's only
    // a hint for Decoder::decode. Most of the structures are
    // dynamically determining their decoding and encoding sizes

    // read_opt - decode with optional paramters. required for complex structure
    // with different origins (save / entfile) like entities
    pub fn read_opt<'o, T: DecoderOpt>(&mut self, size: usize, opt: T::Opt<'o>) -> Result<T> {
        let val = T::decode(&self.raw, self.offset(), size, Some(opt))?;
        self.skip(val.get_enc_size());
        Ok(val)
    }

    pub fn read<T: Decoder>(&mut self, size: usize) -> Result<T> {
        let val = T::decode(&self.raw, self.offset(), size)?;
        self.skip(val.get_enc_size());
        Ok(val)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.rdr.read_u8()?)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(self.rdr.read_u16::<LittleEndian>()?)
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.rdr.read_i32::<LittleEndian>()?)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(self.rdr.read_u32::<LittleEndian>()?)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(self.rdr.read_f32::<LittleEndian>()?)
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

    pub fn skip(&mut self, size: usize) {
        self.buf.set_position(self.buf.position() + size as u64);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.skip(bytes.len());
        self.buf.get_mut().extend(bytes.iter());
    }

    pub fn write<T: Decoder>(&mut self, val: &T) -> Result<()> {
        let mut raw = val.encode()?;
        self.skip(raw.mem.len());
        self.buf.get_mut().append(&mut raw.mem);
        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<()> {
        Ok(self.buf.write_u8(val)?)
    }

    pub fn write_u16(&mut self, val: u16) -> Result<()> {
        Ok(self.buf.write_u16::<LittleEndian>(val)?)
    }

    pub fn write_i32(&mut self, val: i32) -> Result<()> {
        Ok(self.buf.write_i32::<LittleEndian>(val)?)
    }

    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        Ok(self.buf.write_u32::<LittleEndian>(val)?)
    }

    pub fn write_f32(&mut self, val: f32) -> Result<()> {
        Ok(self.buf.write_f32::<LittleEndian>(val)?)
    }
}
