use super::stream::{ReadStream, WriteStream};
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
        let mut rd = ReadStream::new(raw, offset);
        let name: String = rd.read(0)?;
        let version: String = rd.read(0)?;
        Ok(Tag { name, version })
    }

    fn encode(&self) -> Result<Raw> {
        let mut wd = WriteStream::new(self.get_enc_size());
        wd.write(&self.name)?;
        wd.write(&self.version)?; 
        Ok(wd.into_raw(0, self.get_enc_size()))
    }

    fn get_enc_size(&self) -> usize {
        self.name.get_enc_size() + self.version.get_enc_size()
    }
}
