use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "Vake", version, about)]
pub struct Options {
    #[clap(flatten)]
    pub global: GlobalOptions,
}

#[derive(Debug, Parser)]
pub struct GlobalOptions {
    /// Set verbosity level
    #[clap(long("verbose"), short, global(true), parse(from_occurrences))]
    pub verbosity: u8
}