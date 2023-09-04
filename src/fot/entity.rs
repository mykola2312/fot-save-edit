use super::decoder::DecoderCtx;
use super::entitylist::{EntityEncoding, EntityList};
use super::esh::ESH;
use super::raw::Raw;
use super::stream::{ReadStream, WriteStream};
use anyhow::Result;

const NO_FLAGS: u32 = 0;

pub struct Entity {
    pub flags: u32,
    pub type_idx: usize,
    pub esh: ESH,
    enc_size: usize,
}

impl DecoderCtx<&mut EntityList, &EntityList> for Entity {
    fn decode(raw: &Raw, offset: usize, _: usize, ctx: &mut EntityList) -> Result<Self> {
        let mut rd = ReadStream::new(raw, offset);
        Ok(match ctx.get_entity_encoding() {
            EntityEncoding::File => {
                let flags = NO_FLAGS;
                let type_idx = ctx.add_or_get_type(rd.read(0)?);
                let esh: ESH = rd.read(0)?;
                let enc_size = rd.offset() - offset;
                Entity {
                    flags,
                    type_idx,
                    esh,
                    enc_size,
                }
            }
            EntityEncoding::World => {
                let flags = rd.read_u32()?;
                let type_idx = rd.read_u16()? as usize;
                let esh: ESH = rd.read(0)?;
                let enc_size = rd.offset() - offset;
                Entity {
                    flags,
                    type_idx,
                    esh,
                    enc_size,
                }
            }
        })
    }

    fn encode(&self, ctx: &EntityList) -> Result<Raw> {
        let mut wd = WriteStream::new(self.get_enc_size());
        match ctx.get_entity_encoding() {
            EntityEncoding::File => {
                wd.write(ctx.get_type_name(self.type_idx))?;
                wd.write(&self.esh)?;
            }
            EntityEncoding::World => {
                wd.write_u32(self.flags)?;
                wd.write_u16(self.type_idx as u16)?;
                wd.write(&self.esh)?;
            }
        }
        Ok(wd.into_raw(0, 0))
    }

    fn get_enc_size(&self) -> usize {
        self.enc_size
    }
}
