use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Opts {
    #[clap(short, long, action)]
    pub quiet: bool,
}
