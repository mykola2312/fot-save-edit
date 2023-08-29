use super::decoder::Decoder;
use super::raw::Raw;
use anyhow::Result;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use encoding_rs::WINDOWS_1251;

// FString - Fallout

#[derive(Debug, PartialEq)]
pub enum FStringEncoding {
    ANSI,
    WCS2
}

#[derive(Debug)]
pub struct FString {
    pub encoding: FStringEncoding,
    pub enc_len: usize,
    pub str: String
}

impl Decoder for FString {
    fn decode(raw: &Raw, offset: usize, _: usize) -> Result<Self> {
        let mut rdr = Cursor::new(&raw.mem[offset..]);
        let flen = rdr.read_u32::<LittleEndian>()? as usize;
        let len = flen & !(1<<31);
        let start = offset + 4;
        if flen & (1<<31) == 0 { // ANSI
            let (str, _, _) = WINDOWS_1251.decode(&raw.mem[start..start+len]);
            Ok(FString { encoding: FStringEncoding::ANSI, enc_len: len, str: str.to_string() })
        } else { // WCS2
            let chars: Vec<u8> = raw.mem[start..start+len*2]
                .iter().step_by(2).copied().collect();
            let (str, _, _) = WINDOWS_1251.decode(&chars);
            Ok(FString { encoding: FStringEncoding::WCS2, enc_len: len, str: str.to_string() })
        }
    }

    fn encode(&self) -> Result<Raw> {
        let mut buf = vec![0u8; 4];
        let mut wdr = Cursor::new(&mut buf[..]);
        let (chars, _, _) = WINDOWS_1251.encode(self.str.as_str());
        if self.encoding == FStringEncoding::ANSI {
            wdr.write_u32::<LittleEndian>(chars.len() as u32 & !(1<<31))?;
            buf.extend(chars.iter());
        } else { // WCS2
            wdr.write_u32::<LittleEndian>(chars.len() as u32 | (1<<31))?;
            for &c in chars.iter() {
                buf.push(c);
                buf.push(0);
            }
        };
        Ok(Raw { offset: 0, size: buf.len(), mem: buf })
    }

    fn get_enc_size(&self) -> usize {
        4 + self.enc_len
    }
}