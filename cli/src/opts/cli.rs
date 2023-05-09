use crate::commands::upload_abis::UploadAbisArgs;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "unchained")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Unchained CLI, manage your ABI files.",
    after_help = "Find more information to https://unchained.watch/docs",
    next_display_order = None
)]
pub enum Subcommands {
    #[clap(about = "Upload your smart contracts ABI files.")]
    UploadAbis(UploadAbisArgs),
}
