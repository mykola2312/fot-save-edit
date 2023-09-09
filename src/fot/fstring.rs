use super::decoder::Decoder;
use super::ferror::FError as FE;
use super::stream::{ReadStream, WriteStream};
use encoding_rs::WINDOWS_1251;
use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};

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
    fn decode<'a>(rd: &mut ReadStream<'a>) -> Result<Self, FE> {
        //let mut rdr = Cursor::new(&raw.mem[offset..]);
        let flen = rd.read_u32()? as usize;
        let len = flen & !(1 << 31);
        if flen & (1 << 31) == 0 {
            // ANSI
            let bytes = rd.as_bytes(len)?;
            let (str, _, _) = WINDOWS_1251.decode(bytes);
            Ok(FString {
                encoding: FStringEncoding::ANSI,
                enc_len: len,
                str: str.to_string(),
            })
        } else {
            // WCS2
            let bytes = rd.as_bytes(len * 2)?;
            let chars: Vec<u8> = bytes.iter().step_by(2).copied().collect();
            let (str, _, _) = WINDOWS_1251.decode(&chars);
            Ok(FString {
                encoding: FStringEncoding::WCS2,
                enc_len: len,
                str: str.to_string(),
            })
        }
    }

    fn encode(&self, wd: &mut WriteStream) -> Result<(), FE> {
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
