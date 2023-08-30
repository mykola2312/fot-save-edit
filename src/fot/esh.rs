use super::raw::Raw;
use super::tag::Tag;
use super::decoder::Decoder;
use super::fstring::FString;
use super::stream::{ReadStream, WriteStream};
use anyhow::Result;
use indexmap::IndexMap;

pub struct ESHValue {
    pub data_type: u32,
    pub data: Vec<u8>
}

pub struct ESH {
    pub tag: Tag,
    pub props: IndexMap<FString, ESHValue>,
    enc_size: usize
}

impl Decoder for ESH {
    fn decode(raw: &Raw, offset: usize, _: usize) -> Result<Self> {
        let mut rd = ReadStream::new(raw, offset);
        let tag: Tag = rd.read(0)?;

        let n = rd.read_u32()? as usize;
        let mut props: IndexMap<FString, ESHValue> = IndexMap::with_capacity(n);
        for _ in 0..n {
            let name: FString = rd.read(0)?;
            let data_type = rd.read_u32()?;
            let data_size = rd.read_u32()? as usize;
            let data = rd.read_bytes(data_size)?;
            props.insert(name, ESHValue { data_type, data });
        }

        let enc_size = rd.offset() - offset;
        Ok(ESH { tag, props, enc_size })
    }

    fn encode(&self) -> Result<Raw> {
        todo!();
    }

    fn get_enc_size(&self) -> usize {
        self.enc_size
    }
}