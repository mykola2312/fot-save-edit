use super::decoder::Decoder;
use super::raw::Raw;
use super::stream::{ReadStream, WriteStream};
use anyhow::Result;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub version: String,
}

impl Decoder for Tag {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self> {
        let mut rd = ReadStream::new(raw, offset);
        let name: String = rd.read(0)?;
        let version: String = rd.read(0)?;
        Ok(Tag { name, version })
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<()> {
        wd.write(&self.name)?;
        wd.write(&self.version)?;
        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        self.name.get_enc_size() + self.version.get_enc_size()
    }
}

// struct for Tag consts
pub struct CTag<'a> {
    pub name: &'a str,
    pub version: &'a str,
}

impl<'a> CTag<'a> {
    pub fn to_tag(&self) -> Tag {
        Tag {
            name: self.name.to_string(),
            version: self.version.to_string(),
        }
    }
}
