use super::decoder::Decoder;
use super::ferror::FError as FE;
use super::stream::{ReadStream, WriteStream};

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub version: String,
}

impl Decoder for Tag {
    fn decode<'a>(rd: &mut ReadStream<'a>) -> Result<Self, FE> {
        let name: String = rd.read()?;
        let version: String = rd.read()?;
        Ok(Tag { name, version })
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<(), FE> {
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
