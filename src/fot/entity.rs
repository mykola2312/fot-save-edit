use super::attributes::Attributes;
use super::decoder::DecoderCtx;
use super::entitylist::{EntityEncoding, EntityList};
use super::esh::{ESHValue, ESH};
use super::ferror::FError as FE;
use super::stream::{ReadStream, WriteStream};

pub const NO_FLAGS: u32 = 0;
pub const NO_ESH: usize = 0xFFFF;

pub struct Entity {
    pub flags: u32,
    pub type_idx: usize,
    pub esh: Option<ESH>,
    enc_size: usize,
}

impl Entity {
    pub fn get_esh(&self) -> Result<&ESH, FE> {
        match &self.esh {
            Some(esh) => Ok(esh),
            None => Err(FE::EntityNoESH),
        }
    }

    pub fn get_esh_mut(&mut self) -> Result<&mut ESH, FE> {
        match &mut self.esh {
            Some(esh) => Ok(esh),
            None => Err(FE::EntityNoESH),
        }
    }

    pub fn get_attributes(&self) -> Result<Attributes, FE> {
        let value = match self.get_esh()?.get("Attributes") {
            Some(value) => value,
            None => return Err(FE::EntityNoAttributes),
        };

        if let ESHValue::Binary(bin) = value {
            Ok(Attributes::from_binary(&bin)?)
        } else {
            Err(FE::AttributesNonBinary)
        }
    }

    pub fn set_attributes(&mut self, attrs: Attributes) -> Result<(), FE> {
        self.get_esh_mut()?
            .set("Attributes", ESHValue::Binary(attrs.into_binary()?));

        Ok(())
    }
}

impl DecoderCtx<&mut EntityList, &EntityList> for Entity {
    fn decode<'a>(rd: &mut ReadStream<'a>, ctx: &mut EntityList) -> Result<Self, FE> {
        let offset = rd.offset();
        Ok(match ctx.get_entity_encoding() {
            EntityEncoding::File => {
                let flags = NO_FLAGS;
                let type_idx = ctx.add_or_get_type(rd.read()?);
                let esh: ESH = rd.read()?;
                let enc_size = rd.offset() - offset;
                Entity {
                    flags,
                    type_idx,
                    esh: Some(esh),
                    enc_size,
                }
            }
            EntityEncoding::World => {
                let flags = rd.read_u32()?;
                let type_idx = rd.read_u16()? as usize;
                let esh: Option<ESH> = if type_idx != NO_ESH {
                    Some(rd.read()?)
                } else {
                    None
                };

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

    fn encode(&self, wd: &mut WriteStream, ctx: &EntityList) -> Result<(), FE> {
        match ctx.get_entity_encoding() {
            EntityEncoding::File => {
                wd.write(ctx.get_type_name(self.type_idx))?;
                wd.write(self.esh.as_ref().unwrap())?;
            }
            EntityEncoding::World => {
                wd.write_u32(self.flags)?;
                wd.write_u16(self.type_idx as u16)?;
                match self.esh.as_ref() {
                    Some(esh) => wd.write(esh)?,
                    None => (),
                }
            }
        }
        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        self.enc_size
    }
}
