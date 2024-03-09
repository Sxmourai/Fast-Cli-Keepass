use std::{
    io::{self, Write},
    path::PathBuf,
};

use keepass::Database;

// use crate::config::AppConfig;

pub mod args;
// pub mod config;
pub mod db;

fn main() {
    let verbose = 0;
    use clap::Parser;
    let args = args::Args::parse();
    {
        use args::Commands::*;
        match args.command {
            Read { entry } => {
                let pass = if let Some(pass) = args.db_pass {
                    pass
                } else {
                    input("Database key")
                };
                if verbose>0 {
                    println!("Opening db at {}", args.db_path);
                }
                let db = open_db(args.db_path.into(), pass);
                if verbose>0 {
                    println!("Searching {entry} in db...",);
                }
                for ent in get_matching(&db, &entry, 4) {
                    if verbose>0 {
                        println!(
                            "Found \"{}\": {}",
                            ent.get_title().unwrap(),
                            ent.get_password().unwrap()
                        );
                        println!("TODO Clipboard !");
                        dbg!(ent.get_password());
                    } else {
                        println!("{}", ent.get_password().unwrap());
                    }
                }
            }
            // Config => {
            //     let path = input("Database path");
            //     // let path = "/home/sxmourai/Notes obsidian/Files/private/Database.kdbx".to_string();
            //     AppConfig::new(PathBuf::from(path));
            // }
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

fn open_db(db_path: PathBuf, pass: String) -> Database {
    Database::open(
        &mut std::fs::File::open(db_path).unwrap(),
        keepass::DatabaseKey::new().with_password(&pass),
    )
    .expect("Failed opening/parsing database")
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
