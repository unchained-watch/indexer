use clap::Parser;
use cli::opts::{Opts, Subcommands};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    match opts.sub {
        Subcommands::UploadAbis(cmd) => cmd.run()?,
    }

    Ok(())
}
