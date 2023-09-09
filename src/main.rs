#![deny(rust_2018_idioms)]
use std::env;
use std::path::Path;

mod fot;
use fot::save::Save;

fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).unwrap();
    let out_path = match args.get(2) {
        Some(path) => path,
        None => "out.sav"
    };

    let mut save = Save::load(Path::new(save_path)).expect("load save");
    save.world.test().expect("test");
    save.save(Path::new(out_path)).expect("failed to save");
}
