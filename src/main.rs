use clap::Parser;
use log::info;

use crate::{cli::Opts, error::FoobarError};

mod cli;
mod error;

#[tokio::main]
async fn main() -> Result<(), FoobarError> {
    pretty_env_logger::init();

    let _opts: Opts = Opts::parse();

    info!("Initialised!");

    Ok(())
}
