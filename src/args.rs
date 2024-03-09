use clap::Subcommand;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Args {
    pub db_path: String,
    #[arg(long, name = "Put the password as a cli arg")]
    pub im_stupid: Option<String>,
    #[arg(
        long,
        name = "Use the best result if there are multiple matches",
    )]
    pub use_best_result: Option<bool>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    Read { entry: String },
    // Config,
}
// impl clap::ValueEnum for Commands {
//     fn value_variants<'a>() -> &'a [Self] {
//         &[Self::ReadPass, Self::Configure]
//     }

//     fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
//         Some(clap::builder::PossibleValue::new(match self {
//             Self::ReadPass => "read",
//             Self::Configure => "config",
//             #[allow(unreachable_patterns)]
//             _ => None?,
//         }))
//     }
// }
