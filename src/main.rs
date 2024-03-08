pub mod args;

fn main() {
    use clap::Parser;
    let parser = args::Args::parse();
    match parser.command {
        args::Command::ReadPass => todo!()
    }`, `ValueParserFactory`, `From`, and `FromStr
}
