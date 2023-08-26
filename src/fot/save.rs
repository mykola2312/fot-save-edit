use std::str;
use std::fs;
use std::path::Path;
use anyhow::anyhow;
use anyhow::Result;
use crate::fot::raw::Raw;
use crate::fot::world::World;

use super::decoder::Decoder;

#[derive(Debug)]
pub struct Save {
    pub raw: Raw,
    pub world: World
}

impl Save {
    const WORLD_HDR: &str = "<world>";

    pub fn load(path: &Path) -> Result<Self> {
        let raw = Raw::load_file(path)?;
        /*let world_offset = match raw.find_str_backwards(Self::WORLD_HDR) {
            Some(offset) => offset,
            None => return Err(anyhow!("no world found in file"))
        };*/
        let world = World::decode(&raw, 0x99A84, 0x3809B)?;
        Ok(Save { raw, world })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        self.raw.assemble_file(path, vec![
            self.world.encode()
        ])?;

        Ok(())
    }
}