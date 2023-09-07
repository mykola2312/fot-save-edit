use super::decoder::Decoder;
use super::raw::Raw;
use super::stream::WriteStream;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use encoding_rs::WINDOWS_1251;
use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::Cursor;

// FString - Fallout

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum FStringEncoding {
    ANSI,
    WCS2,
}

#[derive(Debug, Eq, Clone)]
pub struct FString {
    pub encoding: FStringEncoding,
    pub enc_len: usize,
    pub str: String,
}

impl Decoder for FString {
    fn decode(raw: &Raw, offset: usize, _: usize) -> Result<Self> {
        let mut rdr = Cursor::new(&raw.mem[offset..]);
        let flen = rdr.read_u32::<LittleEndian>()? as usize;
        let len = flen & !(1 << 31);
        let start = offset + 4;
        if flen & (1 << 31) == 0 {
            // ANSI
            let (str, _, _) = WINDOWS_1251.decode(&raw.mem[start..start + len]);
            Ok(FString {
                encoding: FStringEncoding::ANSI,
                enc_len: len,
                str: str.to_string(),
            })
        } else {
            // WCS2
            let chars: Vec<u8> = raw.mem[start..start + len * 2]
                .iter()
                .step_by(2)
                .copied()
                .collect();
            let (str, _, _) = WINDOWS_1251.decode(&chars);
            Ok(FString {
                encoding: FStringEncoding::WCS2,
                enc_len: len,
                str: str.to_string(),
            })
        }
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<()> {
        let (chars, _, _) = WINDOWS_1251.encode(self.str.as_str());
        if self.encoding == FStringEncoding::ANSI {
            wd.write_u32(chars.len() as u32 & !(1 << 31))?;
            wd.write_bytes(&chars);
        } else {
            // WCS2
            wd.write_u32(chars.len() as u32 | (1 << 31))?;
            for &c in chars.iter() {
                wd.write_u8(c)?;
                wd.write_u8(0)?;
            }
        };
        Ok(())
    }

    fn get_enc_size(&self) -> usize {
        4 + match self.encoding {
            FStringEncoding::ANSI => self.enc_len,
            FStringEncoding::WCS2 => self.enc_len * 2,
        }
    }
}

impl PartialEq for FString {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

impl std::cmp::PartialEq<str> for FString {
    fn eq(&self, other: &str) -> bool {
        self.str == other
    }
}

impl Hash for FString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.str.hash(state);
    }
}

impl Borrow<str> for FString {
    fn borrow(&self) -> &str {
        self.str.as_str()
    }
}

impl fmt::Display for FString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}
