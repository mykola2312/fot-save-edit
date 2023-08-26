use anyhow::Result;
use crate::fot::raw::Raw;
use crate::fot::decoder::Decoder;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub version: String
}

impl Decoder for Tag {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self> {
        let name = String::decode(raw, offset, size)?;
        let version = String::decode(raw, offset + name.len()+1, 0)?;
        Ok(Tag {name, version})
    }

    fn encode(&self) -> Raw {
        let len = self.name.len() + 1 + self.version.len() + 1;
        Raw::join(0, len, &mut [
            self.name.encode(),
            self.version.encode()
        ])
    }
}