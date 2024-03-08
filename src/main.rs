pub mod args;
pub mod db;
pub mod config;

fn main() {
    config::ensure_created().unwrap();
    use clap::Parser;
    let parser = args::Args::parse();
    {
        use args::Command::*;
        match parser.command {
            ReadPass => todo!(),
            Configure => {},
            #[allow(unreachable_patterns)]
            _ => todo!()
        }
    }
}
