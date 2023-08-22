use std::fs;
use std::str;
use std::path::Path;
use anyhow::anyhow;
use anyhow::Result;
use inflate::inflate_bytes_zlib;

#[derive(Debug)]
pub struct World {
    offset: usize,
    size: usize,
    data: Vec<u8>
}

impl World {
    const DATA_OFFSET: usize = 0x13;

    fn decode(raw: &[u8], offset: usize, size: usize) -> Result<Self> {
        let data_offset = offset + World::DATA_OFFSET;
        let data = inflate_bytes_zlib(&raw[data_offset..data_offset+size])
            .map_err(|e| anyhow!(e))?;

        Ok(Self { offset, size, data })
    }

    pub fn dump(&self, path: &Path) -> Result<()> {
        Ok(fs::write(path, &self.data)?)
    }
}

#[derive(Debug)]
pub struct Save {
    raw: Vec<u8>,
    worlds: Vec<World>
}

impl Save {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read(path)?;
        let file_end = raw.len() - 1;
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

        let mut worlds: Vec<World> = Vec::new();
        for i in offsets.chunks(2) {
            let offset = i[0];
            let size = i.get(1).unwrap_or(&file_end);
            match World::decode(&raw, offset, *size) {
                Ok(world) => worlds.push(world),
                Err(e) => println!("world 0x{:x} decode error {}", offset, e)
            };
        }

        Ok(Self { raw, worlds })
    }
}