use std::env;
use std::str;
use std::fs;
use inflate::inflate_bytes_zlib;
pub mod save;

fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).unwrap();
    let out_path = match args.get(2) {
        Some(path) => path,
        None => "out.bin"
    };
    
    let save_buf = fs::read(save_path).unwrap();
    let size = save_buf.len();

    let mut offset = size;
    for i in (0..size - 7).rev() {
        let header = match str::from_utf8(&save_buf[i..i+7]) {
            Ok(header) => header,
            Err(_) => continue
        };

        if header == "<world>" {
            offset = i;
            break;
        }
    }

    if offset == size {
        panic!("Didn't find any world header");
    } else {
        println!("Found offset 0x{:x}", offset);
    }

    // offset+0x13 - deflate
    match inflate_bytes_zlib(&save_buf[offset+0x13..]) {
        Ok(out_buf) => fs::write(out_path, &out_buf).expect("out_buf"),
        Err(e) => panic!("{}", e)
    };
}