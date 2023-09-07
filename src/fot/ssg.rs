use super::decoder::Decoder;
use super::raw::Raw;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::Result;

#[derive(Debug)]
pub struct SSG {
    tag: Tag,
    unk1: Vec<u8>,
}

impl Decoder for SSG {
    fn decode(raw: &Raw, offset: usize, _: usize) -> Result<Self> {
        let mut rd = ReadStream::new(raw, offset);
        let tag: Tag = rd.read(0)?;
        let unk1 = rd.read_bytes(0x14)?;
        Ok(SSG { tag, unk1 })
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<()> {
        wd.write(&self.tag)?;
        wd.write_bytes(&self.unk1);
        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        self.tag.get_enc_size() + 0x14
    }
}
