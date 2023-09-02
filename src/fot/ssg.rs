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
    type Opt<'o> = ();
    fn decode(raw: &Raw, offset: usize, _: usize, _: Option<()>) -> Result<Self> {
        let mut rd = ReadStream::new(raw, offset);
        let tag: Tag = rd.read(0)?;
        let unk1 = rd.read_bytes(0x14)?;
        Ok(SSG { tag, unk1 })
    }

    fn encode(&self) -> Result<Raw> {
        let mut wd = WriteStream::new(self.get_enc_size());
        wd.write(&self.tag)?;
        wd.write_bytes(&self.unk1);
        Ok(wd.into_raw(0, 0x14))
    }

    fn get_enc_size(&self) -> usize {
        self.tag.get_enc_size() + 0x14
    }
}
