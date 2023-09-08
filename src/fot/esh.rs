use super::decoder::Decoder;
use super::fstring::FString;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::Result;
use indexmap::IndexMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct ESHUnknown {
    pub data_type: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ESHEntityFlags {
    pub entity_id: u16,
    pub flags: u16,
}

impl ESHEntityFlags {
    const SIZE: usize = 4;
}

#[derive(Debug, PartialEq)]
pub struct ESHFrame {
    pub unk1: Vec<u8>,
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl ESHFrame {
    const SIZE: usize = 48;
}

#[derive(Debug, Eq, PartialEq)]
pub struct ESHRect {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
}

impl ESHRect {
    const SIZE: usize = 16;
}

#[derive(Debug, PartialEq)]
pub enum ESHValue {
    Unknown(ESHUnknown),
    Bool(bool),
    Float(f32),
    Int(i32),
    String(FString),
    Sprite(FString),
    Enum(FString),
    Binary(Vec<u8>),
    EntityFlags(ESHEntityFlags),
    Frame(ESHFrame),
    Rect(ESHRect),
}

impl ESHValue {
    const HDR_SIZE: usize = 8;

    const TYPE_BOOL: u32 = 1;
    const TYPE_FLOAT: u32 = 2;
    const TYPE_INT: u32 = 3;
    const TYPE_STRING: u32 = 4;
    const TYPE_SPRITE: u32 = 8;
    const TYPE_ENUM: u32 = 9;
    const TYPE_ESBIN: u32 = 11;
    const TYPE_ENTTITYFLAGS: u32 = 12;
    const TYPE_FRAME: u32 = 13;
    const TYPE_RECT: u32 = 14;
}

impl Decoder for ESHValue {
    fn decode<'a>(rd: &mut ReadStream<'a>) -> Result<Self> {
        let data_type = rd.read_u32()?;
        let data_size = rd.read_u32()?;

        Ok(match data_type {
            Self::TYPE_BOOL => ESHValue::Bool(rd.read_u8()? == 1),
            Self::TYPE_FLOAT => ESHValue::Float(rd.read_f32()?),
            Self::TYPE_INT => ESHValue::Int(rd.read_i32()?),
            Self::TYPE_STRING => ESHValue::String(rd.read::<FString>()?),
            Self::TYPE_SPRITE => ESHValue::Sprite(rd.read::<FString>()?),
            Self::TYPE_ENUM => ESHValue::Enum(rd.read::<FString>()?),
            Self::TYPE_ESBIN => ESHValue::Binary(rd.read_bytes(data_size as usize)?),
            Self::TYPE_ENTTITYFLAGS => {
                let entity_id = rd.read_u16()?;
                let flags = rd.read_u16()?;
                ESHValue::EntityFlags(ESHEntityFlags { entity_id, flags })
            }
            Self::TYPE_FRAME => {
                let unk1 = rd.read_bytes(0x24)?;
                let c = rd.read_f32()? * 4.;
                let b = rd.read_f32()? * 4.;
                let a = rd.read_f32()? * 4.;
                ESHValue::Frame(ESHFrame { unk1, a, b, c })
            }
            Self::TYPE_RECT => {
                let top = rd.read_i32()?;
                let left = rd.read_i32()?;
                let right = rd.read_i32()?;
                let bottom = rd.read_i32()?;
                ESHValue::Rect(ESHRect {
                    top,
                    left,
                    right,
                    bottom,
                })
            }
            _ => {
                let data = rd.read_bytes(data_size as usize)?;
                ESHValue::Unknown(ESHUnknown { data_type, data })
            }
        })
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<()> {
        match self {
            ESHValue::Unknown(unk) => {
                wd.write_u32(unk.data_type)?;
                wd.write_u32(unk.data.len() as u32)?;

                wd.write_bytes(&unk.data);
            }
            ESHValue::Bool(val) => {
                wd.write_u32(Self::TYPE_BOOL)?;
                wd.write_u32(1)?;

                wd.write_u8(*val as u8)?;
            }
            ESHValue::Float(val) => {
                wd.write_u32(Self::TYPE_FLOAT)?;
                wd.write_u32(4)?;

                wd.write_f32(*val)?;
            }
            ESHValue::Int(val) => {
                wd.write_u32(Self::TYPE_INT)?;
                wd.write_u32(4)?;

                wd.write_i32(*val)?;
            }
            ESHValue::String(str) => {
                wd.write_u32(Self::TYPE_STRING)?;
                wd.write_u32(str.get_enc_size() as u32)?;

                wd.write(str)?;
            }
            ESHValue::Sprite(spr) => {
                wd.write_u32(Self::TYPE_SPRITE)?;
                wd.write_u32(spr.get_enc_size() as u32)?;

                wd.write(spr)?;
            }
            ESHValue::Enum(spr) => {
                wd.write_u32(Self::TYPE_ENUM)?;
                wd.write_u32(spr.get_enc_size() as u32)?;

                wd.write(spr)?;
            }
            ESHValue::Binary(bin) => {
                wd.write_u32(Self::TYPE_ESBIN)?;
                wd.write_u32(bin.len() as u32)?;

                wd.write_bytes(bin);
            }
            ESHValue::EntityFlags(eflags) => {
                wd.write_u32(Self::TYPE_ENTTITYFLAGS)?;
                wd.write_u32(ESHEntityFlags::SIZE as u32)?;

                wd.write_u16(eflags.entity_id)?;
                wd.write_u16(eflags.flags)?;
            }
            ESHValue::Frame(frame) => {
                wd.write_u32(Self::TYPE_FRAME)?;
                wd.write_u32(ESHFrame::SIZE as u32)?;

                wd.write_bytes(&frame.unk1);
                wd.write_f32(frame.c / 4.)?;
                wd.write_f32(frame.b / 4.)?;
                wd.write_f32(frame.a / 4.)?;
            }
            ESHValue::Rect(rect) => {
                wd.write_u32(Self::TYPE_RECT)?;
                wd.write_u32(ESHRect::SIZE as u32)?;

                wd.write_i32(rect.top)?;
                wd.write_i32(rect.left)?;
                wd.write_i32(rect.right)?;
                wd.write_i32(rect.bottom)?;
            }
        };

        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        Self::HDR_SIZE
            + match self {
                ESHValue::Unknown(unk) => unk.data.len(),
                ESHValue::Bool(_) => 1,
                ESHValue::Float(_) => 4,
                ESHValue::Int(_) => 4,
                ESHValue::String(str) => str.get_enc_size(),
                ESHValue::Sprite(spr) => spr.get_enc_size(),
                ESHValue::Enum(enm) => enm.get_enc_size(),
                ESHValue::Binary(bin) => bin.len(),
                ESHValue::EntityFlags(_) => ESHEntityFlags::SIZE,
                ESHValue::Frame(_) => ESHFrame::SIZE,
                ESHValue::Rect(_) => ESHRect::SIZE,
            }
    }
}

impl fmt::Display for ESHValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ESHValue::Unknown(unk) => {
                write!(f, "Unknown type {}, size {}", unk.data_type, unk.data.len())
            }
            ESHValue::Bool(val) => write!(f, "{}", val),
            ESHValue::Float(val) => write!(f, "{}", val),
            ESHValue::Int(val) => write!(f, "{}", val),
            ESHValue::String(str) => write!(f, "{}", str),
            ESHValue::Sprite(spr) => write!(f, "{}", spr),
            ESHValue::Enum(enm) => write!(f, "{}", enm),
            ESHValue::Binary(bin) => write!(f, "Binary, size {}", bin.len()),
            ESHValue::EntityFlags(val) => {
                write!(f, "entity {} flags {:x}", val.entity_id, val.flags)
            }
            ESHValue::Frame(val) => {
                write!(f, "[{},{},{}]", val.a, val.b, val.c)
            }
            ESHValue::Rect(val) => {
                write!(
                    f,
                    "[({},{}),({},{})]",
                    val.top, val.left, val.right, val.bottom
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct ESH {
    pub tag: Tag,
    pub props: IndexMap<FString, ESHValue>,
    enc_size: usize,
}

impl ESH {
    pub fn get(&self, name: &str) -> Option<&ESHValue> {
        self.props.get(name)
    }

    pub fn set(&mut self, name: &str, value: ESHValue) {
        self.props[name] = value;
    }
}

impl Decoder for ESH {
    fn decode<'a>(rd: &mut ReadStream<'a>) -> Result<Self> {
        let offset = rd.offset();
        let tag: Tag = rd.read()?;

        let n = rd.read_u32()? as usize;
        let mut props: IndexMap<FString, ESHValue> = IndexMap::with_capacity(n);
        for _ in 0..n {
            let name: FString = rd.read()?;
            let value: ESHValue = rd.read()?;
            props.insert(name, value);
        }

        let enc_size = rd.offset() - offset;
        Ok(ESH {
            tag,
            props,
            enc_size,
        })
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<()> {
        wd.write(&self.tag)?;

        wd.write_u32(self.props.len() as u32)?;
        for (name, value) in self.props.iter() {
            wd.write(name)?;
            wd.write(value)?;
        }

        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        self.enc_size
    }
}
