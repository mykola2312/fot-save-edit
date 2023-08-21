use std::fs;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;
use inflate::inflate_bytes_zlib;

struct World {
    offset: usize,
    size: usize,
    data: Vec<u8>
}

#[derive(Error, Debug)]
enum WorldDecodeError {
    #[error("inflate error")]
    InflateError(String)
}

impl World {
    fn decode(raw: &[u8], offset: usize, size: usize) -> Result<Self, WorldDecodeError> {
        let data = match inflate_bytes_zlib(&raw[offset..offset+size]) {
            Ok(data) => data,
            Err(e) => return Err(WorldDecodeError::InflateError((e)))
        };

        Ok(Self { offset, size, data })
    }
}

struct Save {
    raw: Vec<u8>
}

#[derive(Error, Debug)]
enum SaveLoadError {
    #[error("file error")]
    FileError(std::io::Error)
}

impl Save {
    fn load(path: &Path) -> Result<Self, SaveLoadError> {
        let raw = match fs::read(path) {
            Ok(raw) => raw,
            Err(e) => return Err(SaveLoadError::FileError(e))
        };

        Ok(Self { raw })
    }
}