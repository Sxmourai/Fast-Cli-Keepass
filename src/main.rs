use std::{
    io::{self, Write},
    path::PathBuf,
};
use clap::Parser;

use keepass::Database;

use color_eyre::{eyre::{Context as _, Error}, Result};

pub mod args;
// pub mod config;
pub mod db;
use args::Commands::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    let verbose = 0;
    let args = args::Args::parse();
    match args.command {
        Read { entry } => {
            let pass = if let Some(pass) = args.im_stupid {
                pass
            } else {
                rpassword::prompt_password("Database key: ").unwrap()
            };
            if verbose > 0 {
                println!("Opening db at {}", args.db_path);
            }
            let db = open_db(args.db_path.into(), pass)?;
            if verbose > 0 {
                println!("Searching {entry} in db...",);
            }
            let mut entries = get_matching(&db, &entry);
            entries.sort_by(|(s1,_), (s2,_)|s1.total_cmp(s2));
            let entries = entries.into_iter().filter(|(s,_)|*s>0.5).collect::<Vec<(f32, &keepass::db::Entry)>>();
            if entries.len()==0 {
                return Err(color_eyre::Report::msg("Found no matches"))
            }
            if entries.len()>1 {
                return Err(color_eyre::Report::msg("TODO: Support multiple results"))
            }
            for (_, ent) in entries {
                if verbose > 0 {
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
    Ok(())
}

fn input(msg: impl std::fmt::Display) -> String {
    print!("{}: ", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.strip_suffix("\n").unwrap().to_string()
}

fn open_db(db_path: PathBuf, pass: String) -> Result<Database> {
    Ok(Database::open(
        &mut std::fs::File::open(&db_path)
            .wrap_err_with(|| format!("Failed to read database from {}", db_path.display()))?,
        keepass::DatabaseKey::new().with_password(&pass),
    )?)
}

/// Returns the scores of the entries
fn get_matching<'a>(
    db: &'a keepass::Database,
    query: &str,
) -> Vec<(f32, &'a keepass::db::Entry)> {
    db.root
        .iter()
        .map(|child| {
            match child {
                keepass::db::NodeRef::Group(_grp) => None, //todo!("parse {:?}",grp),
                keepass::db::NodeRef::Entry(ent) => {
                    let score = rust_fuzzy_search::fuzzy_compare(query, &format!("{}{}{}",ent.get_title().unwrap(),ent.get_username().unwrap(),ent.get_url().unwrap()));
                    return Some((score, ent));
                }
            }
        })
        .filter(|ent| ent.is_some())
        .map(|ent| ent.unwrap())
        .collect()
}
