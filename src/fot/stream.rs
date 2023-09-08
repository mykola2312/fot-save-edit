use super::decoder::{Decoder, DecoderCtx};
use super::raw::Raw;
use anyhow::anyhow;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub struct ReadStream<'a> {
    mem: &'a [u8],
    rdr: Cursor<&'a [u8]>,
}

impl<'a> ReadStream<'a> {
    pub fn new(mem: &'a [u8], offset: usize) -> ReadStream<'a> {
        let mut rdr = Cursor::new(mem);
        rdr.set_position(offset as u64);
        ReadStream { mem, rdr }
    }

    pub fn offset(&self) -> usize {
        self.rdr.position() as usize
    }

    pub fn skip(&mut self, size: usize) {
        self.rdr.set_position(self.rdr.position() + size as u64);
    }

    pub fn size(&self) -> usize {
        self.mem.len()
    }

    pub fn is_end(&self) -> bool {
        self.offset() >= self.size()
    }

    pub fn as_byte_arr(&self) -> &[u8] {
        &self.mem[self.offset()..]
    }

    pub fn as_bytes(&mut self, size: usize) -> Result<&'a [u8]> {
        if self.offset() + size > self.mem.len() {
            dbg!(self.offset(), size, self.mem.len());
            Err(anyhow!("as_bytes/read_bytes size is bigger than buffer"))
        } else {
            let buf = &self.mem[self.offset()..self.offset() + size];
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
    pub fn read_ctx<T: DecoderCtx<DCtx, ECtx>, DCtx, ECtx>(&mut self, ctx: DCtx) -> Result<T> {
        Ok(T::decode(self, ctx)?)
    }

    pub fn read<T: Decoder>(&mut self) -> Result<T> {
        Ok(T::decode(self)?)
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

    pub fn reserve(&mut self, size: usize) {
        self.buf.get_mut().reserve(size);
    }

    pub fn write_ctx<T: DecoderCtx<DCtx, ECtx>, DCtx, ECtx>(
        &mut self,
        val: &T,
        ctx: ECtx,
    ) -> Result<()> {
        self.reserve(val.get_enc_size());
        val.encode(self, ctx)?;
        Ok(())
    }

    pub fn write<T: Decoder>(&mut self, val: &T) -> Result<()> {
        self.reserve(val.get_enc_size());
        val.encode(self)?;
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
