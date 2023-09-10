#![deny(rust_2018_idioms)]
use clap::{Parser, Subcommand, ValueEnum};
use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};
use std::path::Path;

mod fot;
use fot::attributes::*;
use fot::entity::Entity;
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
        find: String,
    },
    ListValues {
        ids: Option<String>,
        find: Option<String>,
    },
}

fn log_entities<'a>(entlist: &EntityList, iter: impl IntoIterator<Item = (usize, &'a Entity)>) {
    let mut bf = BufWriter::new(stdout().lock());
    for (id, ent) in iter {
        let type_name = if ent.type_idx != 0xFFFF {
            entlist.get_type_name(ent.type_idx).str.as_str()
        } else {
            "<no type>"
        };
        write!(bf, "{}\t{}\n", id, type_name).expect("failed to write stdout");
    }
}

fn parse_kv(kv: &String) -> Vec<(&str, &str)> {
    kv.split(",")
        .map(|kv| kv.split_once("="))
        .collect::<Option<Vec<(&str, &str)>>>()
        .unwrap()
}

fn from_ids(entlist: &EntityList, line: String) -> HashMap<usize, &Entity> {
    line.split(",")
        .map(|id| {
            (
                id.parse::<usize>().expect("parse id"),
                entlist.get_entity(id.parse().expect("parse id")),
            )
        })
        .collect::<HashMap<usize, &Entity>>()
}

fn find_entities(entlist: &EntityList, line: String) -> HashMap<usize, &Entity> {
    let kv = parse_kv(&line);
    let mut entities: HashMap<usize, &Entity> = HashMap::new();
    for (id, ent) in entlist {
        let esh = match &ent.esh {
            Some(esh) => esh,
            None => continue,
        };

        for (name, value) in &esh.props {
            let key = name.str.as_str();
            let svalue = value.to_string();
            for (k, v) in &kv {
                if key == *k && svalue == *v {
                    entities.insert(id, ent);
                }
            }
        }
    }

    entities
}

fn get_entities(
    entlist: &EntityList,
    ids: Option<String>,
    find: Option<String>,
) -> HashMap<usize, &Entity> {
    if let Some(ids) = ids {
        from_ids(entlist, ids)
    } else if let Some(find) = find {
        find_entities(entlist, find)
    } else {
        panic!("No entity selector provided!")
    }
}

fn do_save(cli: Cli) {
    let mut save = match Save::load(Path::new(cli.input.as_str())) {
        Ok(save) => save,
        Err(fe) => panic!("{}", fe),
    };
    let entlist = &save.world.entlist;

    match cli.command {
        Commands::ListEntities => {
            log_entities(entlist, entlist.into_iter());
        }
        Commands::FindEntities { find } => {
            log_entities(entlist, find_entities(entlist, find));
        }
        Commands::ListValues { ids, find } => {
            let entities = get_entities(entlist, ids, find);
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
