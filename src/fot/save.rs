use std::io::Write;
use std::str;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use anyhow::anyhow;
use anyhow::Result;
use inflate::inflate_bytes_zlib;
use deflate::deflate_bytes_zlib;
use crate::fot::decoder::Decoder;
use crate::fot::raw::Raw;
use crate::fot::decoder;

#[derive(Debug)]
pub struct World {
    pub offset: usize,
    pub size: usize,
    pub data: Vec<u8>
}

impl World {
    const DATA_OFFSET: usize = 0x13;

    fn decode(raw: &[u8], offset: usize, size: usize) -> Result<Self> {
        let data_start = offset + World::DATA_OFFSET;
        let data_end = offset + size;
        let data = inflate_bytes_zlib(&raw[data_start..data_end])
            .map_err(|e| anyhow!(e))?;

        Ok(Self { offset, size, data })
    }

    fn encode(&self) -> Vec<u8> {
        deflate_bytes_zlib(&self.data)
    }

    pub fn dump(&self, path: &Path) -> Result<()> {
        Ok(fs::write(path, &self.data)?)
    }
}

#[derive(Debug)]
pub struct Save {
    pub raw: Vec<u8>,
    pub worlds: Vec<World>
    //world: World
}

impl Save {
    const WORLD_HDR: &str = "<world>";
    const WORLD_HDR_LEN: usize = Self::WORLD_HDR.len();

    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read(path)?;
        let file_end = raw.len();
        let mut offsets: Vec<usize> = Vec::new();
        for i in 0..raw.len()-Self::WORLD_HDR_LEN {
            let keyword = match str::from_utf8(&raw[i..i+Self::WORLD_HDR_LEN]) {
                Ok(keyword) => keyword,
                Err(_) => continue
            };

            if keyword == Self::WORLD_HDR {
                offsets.push(i);
            }
        }
        if offsets.is_empty() {
            return Err(anyhow!("no offsets found"));
        }
        offsets.push(file_end);

        let mut worlds: Vec<World> = Vec::new();
        /*for i in offsets.windows(2) {
            match World::decode(&raw, i[0], i[1] - i[0]) {
                Ok(world) => worlds.push(world),
                Err(e) => println!("world 0x{:x} decode error {}", i[0], e)
            };
        }*/

        Ok(Self { raw, worlds })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let raw = Raw { offset: 0, size: self.raw.len(), mem: self.raw.clone() };
        println!("found world at {:x}", raw.find_str_backwards("<world>").unwrap());


        const START: usize = 0x99A84;
        const END: usize = 0xD1B1E; //0xD1B1E;
        const SIZE: usize = 0x38088;
        //let world = self.worlds.last().unwrap();
        let world = World::decode(&self.raw, START, END - START)?;
        let enc = world.encode();

        let mut blocks: Vec<Raw> = Vec::new();
        blocks.push(Raw {offset: START, size: 0x13, mem: self.raw[START..START+0x13].to_vec()});
        blocks.push(Raw {offset: START+0x13, size: SIZE, mem: enc});

        raw.assemble_file(path, blocks)?;

        Ok(())
    }

    pub fn test(&self) -> Result<()> {
        let a = "hello".to_string().encode();
        dbg!(a);

        Ok(())
    }
}