#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    pub command: Command
}


#[derive(Clone, Debug)]
pub enum Command {
    ReadPass,
}
impl clap::ValueEnum for Command {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::ReadPass]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::ReadPass => Some(clap::builder::PossibleValue::new("read")),
        }
    }
}