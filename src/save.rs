use std::fs;
use std::str;
use std::path::Path;
use anyhow::anyhow;
use anyhow::Result;
use inflate::inflate_bytes_zlib;

struct World {
    offset: usize,
    size: usize,
    data: Vec<u8>
}

impl World {
    fn decode(raw: &[u8], offset: usize, size: usize) -> Result<Self> {
        let data = inflate_bytes_zlib(&raw[offset..offset+size])
            .map_err(|e| anyhow!(e))?;

        Ok(Self { offset, size, data })
    }
}

struct Save {
    raw: Vec<u8>,
    worlds: Vec<World>
}

impl Save {
    fn load(path: &Path) -> Result<Self> {
        let raw = fs::read(path)?;
        let mut offsets: Vec<usize> = Vec::new();
        for i in 0..raw.len()-7 {
            let keyword = match str::from_utf8(&raw[i..i+7]) {
                Ok(keyword) => keyword,
                Err(_) => continue
            };

            if keyword == "<world>" {
                offsets.push(i);
            }
        }

        Ok(Self { raw, worlds: Vec::new() })
    }
}