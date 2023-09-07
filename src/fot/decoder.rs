use super::raw::Raw;
use super::stream::WriteStream;
use anyhow::Result;
use std::str;

pub trait Decoder: Sized {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self>;
    fn encode(&self, wd: &mut WriteStream) -> Result<()>;
    fn get_enc_size(&self) -> usize;
}

pub trait DecoderCtx<DCtx, ECtx>: Sized {
    fn decode(raw: &Raw, offset: usize, size: usize, ctx: DCtx) -> Result<Self>;
    fn encode(&self, ctx: ECtx) -> Result<Raw>;
    fn get_enc_size(&self) -> usize;
}

impl Decoder for String {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self> {
        let str = &raw.mem[offset..];
        match str.iter().position(|&c| c == 0) {
            Some(pos) => Ok(str::from_utf8(&str[..pos])?.to_string()),
            None => Ok(str::from_utf8(&raw.mem[offset..offset + size])?.to_string()),
        }
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<()> {
        wd.write_bytes(self.as_bytes());
        wd.write_u8(0)?;
        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        self.len() + 1
    }
}
