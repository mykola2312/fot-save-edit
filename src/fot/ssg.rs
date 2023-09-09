use super::decoder::Decoder;
use super::ferror::FError as FE;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;

#[derive(Debug)]
pub struct SSG {
    tag: Tag,
    unk1: Vec<u8>,
}

impl Decoder for SSG {
    fn decode<'a>(rd: &mut ReadStream<'a>) -> Result<Self, FE> {
        let tag: Tag = rd.read()?;
        let unk1 = rd.read_bytes(0x14)?;
        Ok(SSG { tag, unk1 })
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<(), FE> {
        wd.write(&self.tag)?;
        wd.write_bytes(&self.unk1);
        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        self.tag.get_enc_size() + 0x14
    }
}
