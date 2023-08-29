use super::raw::Raw;
use anyhow::Result;
use std::str;

pub trait Decoder: Sized {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self>;
    fn encode(&self) -> Result<Raw>;
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

    fn encode(&self) -> Result<Raw> {
        let mut str = self.as_bytes().to_vec();
        str.push(0);
        Ok(Raw {
            offset: 0,
            size: str.len(),
            mem: str,
        })
    }

    fn get_enc_size(&self) -> usize {
        self.len() + 1
    }
}
