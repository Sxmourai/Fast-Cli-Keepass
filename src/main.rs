use std::{
    io::{self, Write},
    path::PathBuf,
};

use crate::config::AppConfig;

pub mod args;
pub mod config;
pub mod db;

fn main() {
    config::ensure_created().unwrap();
    use clap::Parser;
    let parser = args::Args::parse();
    {
        use args::Commands::*;
        match parser.command {
            Read { entry } => {
                let config = AppConfig::read();
                let db = config.db();
                println!("Search {entry} in db...",);
                for ent in get_matching(&db, &entry, 4) {
                    println!(
                        "Found \"{}\": {}",
                        ent.get_title().unwrap(),
                        ent.get_password().unwrap()
                    );
                    println!("TODO Clipboard !");
                    dbg!(ent.get_password());
                }
            }
            Config => {
                let path = input("Database path");
                // let path = "/home/sxmourai/Notes obsidian/Files/private/Database.kdbx".to_string();
                AppConfig::new(PathBuf::from(path));
            }
            #[allow(unreachable_patterns)]
            _ => todo!(),
        }
    }
}

fn input(msg: impl std::fmt::Display) -> String {
    print!("{}: ", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.strip_suffix("\n").unwrap().to_string()
}

fn get_matching<'a>(
    db: &'a keepass::Database,
    to_match: &str,
    min_score: u32,
) -> Vec<&'a keepass::db::Entry> {
    db.root
        .iter()
        .map(|child| {
            match child {
                keepass::db::NodeRef::Group(_grp) => {return None} //todo!("parse {:?}",grp),
                keepass::db::NodeRef::Entry(ent) => {
                    for (_title, val) in [
                        ("title", ent.get_title()),
                        ("username", ent.get_username()),
                        ("url", ent.get_url()),
                    ] {
                        if let Some(val) = val {
                            let mut score = 0;
                            for (_i, (c, c2)) in val.chars().zip(to_match.chars()).enumerate() {
                                if c.to_lowercase().collect::<String>()
                                    == c2.to_lowercase().collect::<String>()
                                {
                                    score += 1
                                }
                            }
                            if score > min_score {
                                return Some(ent);
                            }
                        }
                    }
                    return None
                }
            };
            unreachable!()
        }).filter(|ent|ent.is_some()).map(|ent|ent.unwrap())
        .collect()
}
