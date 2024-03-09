use std::{io::{self, Write}, path::PathBuf};

use crate::config::AppConfig;

pub mod args;
pub mod config;
pub mod db;

fn main() {
    config::ensure_created().unwrap();
    use clap::Parser;
    let parser = args::Args::parse();
    {
        use args::Command::*;
        match parser.command {
            ReadPass => {
                let config = AppConfig::read();
                println!("Db path: {:?}", config.db_path);
            },
            Configure => {
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