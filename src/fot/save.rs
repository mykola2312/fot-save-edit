use super::decoder::Decoder;
use super::raw::Raw;
use super::stream::WriteStream;
use super::world::World;
use anyhow::anyhow;
use anyhow::Result;
use byteorder::{ByteOrder, LittleEndian};
use std::path::Path;
use std::str;

pub struct Save {
    pub raw: Raw,
    pub world: World,
}

impl Save {
    const WORLD_TAG: &str = "<world>";
    const CAMPAIGN_TAG: &str = "<campaign>";

    pub fn load(path: &Path) -> Result<Self> {
        let raw = Raw::load_file(path)?;
        let world_offset = match raw.find_str_backwards(Self::WORLD_TAG) {
            Some(offset) => offset,
            None => return Err(anyhow!("no world found in file")),
        };

        let mut world_size: usize = 0;
        {
            let campaign = match raw.find_str(Self::CAMPAIGN_TAG, world_offset) {
                Some(campaign) => world_offset + campaign,
                None => return Err(anyhow!("no campaign found after world")),
            };

            for i in (campaign - 256..campaign).rev() {
                let fsize = LittleEndian::read_u32(&raw.mem[i..i + 4]);
                if fsize & (1 << 31) != 0 {
                    let size = fsize ^ (1 << 31);
                    if size as usize <= campaign - i {
                        world_size = i - world_offset;
                        break;
                    }
                }
            }
        }
        if world_size == 0 {
            return Err(anyhow!("Unable to determine world block size"));
        }

        let world = World::decode(&raw, world_offset, world_size)?;
        Ok(Save { raw, world })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let raw = {
            let mut wd = WriteStream::new(0);
            wd.write(&self.world)?;
            wd.into_raw(self.world.offset, self.world.size)
        };
        self.raw.assemble_file(path, vec![raw])?;

        Ok(())
    }
}
