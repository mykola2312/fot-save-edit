use std::env;
use std::path::Path;

mod fot;
use fot::save::Save;

fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).unwrap();
    /*let out_path = match args.get(2) {
        Some(path) => path,
        None => "out.bin"
    };*/
    
    
    let save = Save::load(Path::new(save_path)).expect("load save");
    for w in save.worlds.iter() {
        println!("World {:x} size {}", w.offset, w.size);
    }

    //save.save(Path::new("out.sav")).expect("failed to save");
    save.test().expect("test");
}