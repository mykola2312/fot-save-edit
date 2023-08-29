use super::decoder::Decoder;
use super::raw::Raw;
use anyhow::Result;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub version: String,
}

impl Decoder for Tag {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self> {
        let name = String::decode(raw, offset, size)?;
        let version = String::decode(raw, offset + name.len() + 1, 0)?;
        Ok(Tag { name, version })
    }

    fn encode(&self) -> Result<Raw> {
        Ok(Raw::join(
            0,
            self.get_enc_size(),
            &mut [self.name.encode()?, self.version.encode()?],
        ))
    }

    fn get_enc_size(&self) -> usize {
        self.name.get_enc_size() + self.version.get_enc_size()
    }
}
