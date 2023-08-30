use super::decoder::Decoder;
use super::fstring::FString;
use super::raw::Raw;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::Result;
use indexmap::IndexMap;

pub struct SGD {
    tag: Tag,
    unk1: Vec<u8>,
    pub dialogs: IndexMap<FString, Vec<FString>>,
    enc_size: usize,
}

impl Decoder for SGD {
    fn decode(raw: &Raw, offset: usize, size: usize) -> Result<Self> {
        let mut rd = ReadStream::new(raw, offset);
        let tag: Tag = rd.read(0)?;
        let unk1 = rd.read_bytes(0x48)?;
        let mut dialogs: IndexMap<FString, Vec<FString>> = IndexMap::new();

        let n = rd.read_u32()? as usize;
        let mut names: Vec<FString> = Vec::with_capacity(n);
        for _ in 0..n {
            names.push(rd.read::<FString>(0)?);
        }

        let m = rd.read_u32()? as usize;
        assert!(m == n, "SGD m != n");
        for _ in 0..m {
            let k = rd.read_u32()? as usize;
            let mut lines: Vec<FString> = Vec::with_capacity(k);
            for _ in 0..k {
                lines.push(rd.read::<FString>(0)?);
            }

            dialogs.insert(names.remove(m), lines);
        }

        let enc_size = rd.offset() - offset;
        Ok(SGD {
            tag,
            unk1,
            dialogs,
            enc_size,
        })
    }

    fn encode(&self) -> Result<Raw> {
        todo!();
    }

    fn get_enc_size(&self) -> usize {
        todo!()
    }
}
