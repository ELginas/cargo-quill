mod build;
mod new;

use crate::build::Build;
use crate::new::New;
use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// Cargo subcommand to build and test Quill/Feather plugins.
struct CargoQuill {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Build(Build),
    New(New),
}

fn main() -> anyhow::Result<()> {
    let args: CargoQuill = argh::from_env();
    match args.subcommand {
        Subcommand::Build(args) => build::build(args),
        Subcommand::New(args) => new::new(args),
    }
}
