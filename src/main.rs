use clap::Parser;
use std::{
    io::{self, Write},
    path::PathBuf,
};

use keepass::Database;

use color_eyre::{eyre::Context as _, Result};

pub mod args;
// pub mod config;
pub mod db;
use args::Commands::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = args::Args::parse();
    let use_best_result = args.use_best_result.unwrap_or(true);
    match args.command {
        Read { entry } => {
            let pass = if let Some(pass) = args.im_stupid {
                pass
            } else {
                rpassword::prompt_password("Database key: ").unwrap()
            };
            let db = open_db(args.db_path.into(), pass)?;
            let ent = get_best_match(&entry, &db, use_best_result)?;
            println!("{}", ent.get_password().unwrap());
            //TODO Clipboard
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

fn input(msg: impl std::fmt::Display) -> Result<String> {
    print!("{}: ", msg);
    io::stdout().flush()?;
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.strip_suffix("\n").unwrap().to_string())
}

/// Returns the entry containing the best match
fn get_best_match<'a>(
    search: &str,
    db: &'a Database,
    use_best_result: bool,
) -> Result<&'a keepass::db::Entry> {
    let mut entries = get_matching(&db, search)?;
    entries.sort_by(|(s1, _), (s2, _)| s2.total_cmp(s1));
    let entries = entries
        .into_iter()
        .filter(|(s, _)| *s >= 0.5)
        .collect::<Vec<(f32, &keepass::db::Entry)>>();
    if entries.len() == 0 {
        Err(color_eyre::Report::msg("Found no matches"))
    } else if entries.len() > 1 {
        if use_best_result {
            Ok(entries[0].1)
        } else {
            for (score, entry) in entries {
                println!("{score:?} - {}", entry.get_title().unwrap())
            }
            Err(color_eyre::Report::msg("TODO: Support multiple results"))
        }
    } else {
        Ok(entries[0].1)
    }
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
) -> Result<Vec<(f32, &'a keepass::db::Entry)>> {
    Ok(db
        .root
        .iter()
        .map(|child| {
            match child {
                keepass::db::NodeRef::Group(_grp) => None, //todo!("parse {:?}",grp),
                keepass::db::NodeRef::Entry(ent) => {
                    let score = rust_fuzzy_search::fuzzy_compare(
                        query,
                        &format!(
                            "{}{}{}",
                            ent.get_title().unwrap_or(""),
                            ent.get_username().unwrap_or(""),
                            ent.get_url().unwrap_or("")
                        ),
                    );
                    return Some((score, ent));
                }
            }
        })
        .filter(|ent| ent.is_some())
        .map(|ent| ent.unwrap())
        .collect())
}
