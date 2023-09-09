#![deny(rust_2018_idioms)]
use clap::{Parser, Subcommand, ValueEnum};
use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};
use std::path::Path;

mod fot;
use fot::attributes::*;
use fot::entitylist::EntityList;
use fot::save::Save;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file path (.sav or .ent)
    #[arg(short, long)]
    input: String,

    // Specify save file or ent file type
    #[arg(value_enum)]
    kind: Kind,

    /// Output file path
    #[arg(short, long)]
    output: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Kind {
    Save,
    Ent,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ListEntities,
    /// Find entities, kv = key1=value,key2=value2
    FindEntities {
        kv: String,
    },
}

fn list_entities(entlist: &EntityList) {
    let mut bf = BufWriter::new(stdout().lock());
    for (id, ent) in entlist {
        let type_name = if ent.type_idx != 0xFFFF {
            entlist.get_type_name(ent.type_idx).str.as_str()
        } else {
            "<no type>"
        };
        write!(bf, "{}\t{}\n", id, type_name).expect("failed to write stdout");
    }
}

fn find_entities(entlist: &EntityList, line: String) {
    let kv = line
        .split(",")
        .map(|kv| kv.split_once("="))
        .collect::<Option<HashMap<&str, &str>>>()
        .unwrap();
    
    let mut bf = BufWriter::new(stdout().lock());
    for (id, ent) in entlist {
        let type_name = if ent.type_idx != 0xFFFF {
            entlist.get_type_name(ent.type_idx).str.as_str()
        } else {
            "<no type>"
        };

        let esh = match &ent.esh {
            Some(esh) => esh,
            None => continue,
        };

        for (name, value) in &esh.props {
            let key = name.str.as_str();
            if kv.contains_key(key) {
                let svalue = value.to_string();
                if svalue == kv[key] {
                    write!(bf, "{}\t{}\n", id, type_name).expect("failed to write stdout");
                }
            }
        }
    }
}

fn do_save(cli: Cli) {
    let mut save = match Save::load(Path::new(cli.input.as_str())) {
        Ok(save) => save,
        Err(fe) => panic!("{}", fe),
    };

    match cli.command {
        Commands::ListEntities => {
            list_entities(&save.world.entlist);
        },
        Commands::FindEntities { kv } => {
            find_entities(&save.world.entlist, kv);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.kind {
        Kind::Save => do_save(cli),
        Kind::Ent => todo!(),
    }

    //let mut save = Save::load(Path::new(save_path)).expect("load save");
    //save.save(Path::new(out_path)).expect("failed to save");
}
