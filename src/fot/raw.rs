use anyhow::Result;
use memmem::{Searcher, TwoWaySearcher};
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::str;

#[derive(Debug)]
pub struct Raw {
    pub offset: usize,
    pub size: usize,
    pub mem: Vec<u8>,
}

impl Raw {
    pub fn join(offset: usize, size: usize, raws: &mut [Raw]) -> Raw {
        let mut mem: Vec<u8> = Vec::new();
        for raw in raws.iter_mut() {
            mem.append(&mut raw.mem);
        }

        Raw {
            offset: offset,
            size: size,
            mem: mem,
        }
    }

    pub fn find_str(&self, str: &str, offset: usize) -> Option<usize> {
        let search = TwoWaySearcher::new(str.as_bytes());
        search.search_in(&self.mem[offset..])
    }

    pub fn find_str_backwards(&self, str: &str) -> Option<usize> {
        for i in (0..self.mem.len() - str.len()).step_by(1024).rev() {
            match self.find_str(str, i) {
                Some(offset) => return Some(i + offset),
                None => continue,
            };
        }

        None
    }

    pub fn load_file(path: &Path) -> Result<Raw> {
        let mem = fs::read(path)?;

        Ok(Self {
            offset: 0,
            size: mem.len(),
            mem,
        })
    }

    pub fn assemble_file(&self, path: &Path, blocks: Vec<Raw>) -> Result<()> {
        let mut file = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path)?,
        );

        let mut sorted = blocks;
        sorted.sort_by(|a, b| a.offset.cmp(&b.offset));

        let file_end = self.size;
        let mut prev_end: usize = 0;
        for block in sorted.iter() {
            // prev
            file.write(&self.mem[prev_end..block.offset])?;
            // data
            file.write(&block.mem)?;
            // padding
            if block.size > block.mem.len() {
                for _ in 0..block.size - block.mem.len() {
                    file.write(&[0])?;
                }
            }
            prev_end = block.offset + block.size;
        }
        if prev_end < file_end {
            file.write(&self.mem[prev_end..file_end])?;
        }

        file.flush()?;
        Ok(())
    }
}
