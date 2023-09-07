use super::stream::{ReadStream, WriteStream};
use anyhow::{anyhow, Result};
use std::str;

pub trait Decoder: Sized {
    fn decode<'a>(rd: &mut ReadStream<'a>) -> Result<Self>;
    fn encode(&self, wd: &mut WriteStream) -> Result<()>;
    fn get_enc_size(&self) -> usize;
}

pub trait DecoderCtx<DCtx, ECtx>: Sized {
    fn decode<'a>(rd: &mut ReadStream<'a>, ctx: DCtx) -> Result<Self>;
    fn encode(&self, wd: &mut WriteStream, ctx: ECtx) -> Result<()>;
    fn get_enc_size(&self) -> usize;
}

impl Decoder for String {
    fn decode<'a>(rd: &mut ReadStream<'a>) -> Result<Self> {
        let bytes = rd.as_byte_arr();
        let pos = match bytes.iter().position(|&c| c == 0) {
            Some(pos) => pos,
            None => return Err(anyhow!("No zero-terminator found"))
        };
        let str = str::from_utf8(rd.as_bytes(pos)?)?;
        rd.skip(1);
        Ok(str.to_string())
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
