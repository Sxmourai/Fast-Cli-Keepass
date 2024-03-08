#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    pub command: Command
}


#[derive(Clone, Debug)]
pub enum Command {
    ReadPass,
    Configure,
}
impl clap::ValueEnum for Command {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::ReadPass,
            Self::Configure,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(match self {
            Self::ReadPass => "read",
            Self::Configure => "config",
            #[allow(unreachable_patterns)]
            _ => None?,
        }))
    }
}