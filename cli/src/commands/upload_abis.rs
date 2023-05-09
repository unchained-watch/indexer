use clap::Args;
use reqwest::Client;
use std::error::Error;
use std::path::Path;
use std::{fs, path::PathBuf};

use crate::api;

#[derive(Args, Debug)]
pub struct UploadAbisArgs {
    #[clap(
        long,
        long_help = "The ABI's root path.",
        value_name = "PATH",
        help = "The ABI's root path."
    )]
    pub path: PathBuf,

    #[clap(
        short,
        long,
        value_name = "GLOB_PATTERN",
        help = "Exclude files that match the pattern (Unix glob syntax)."
    )]
    pub exclude: Option<Vec<String>>,
}

impl UploadAbisArgs {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let client = Client::new();

        for entry in fs::read_dir(self.path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::directory_recursive(&client, &path, &self.exclude)?;
            } else {
                api::upload_file(&client, &path, &self.exclude)?;
            }
        }
        Ok(())
    }

    fn directory_recursive(
        client: &Client,
        dir_path: &Path,
        exclude: &Option<Vec<String>>,
    ) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::directory_recursive(client, &path, exclude)?;
            } else {
                api::upload_file(client, &path, exclude)?;
            }
        }

        Ok(())
    }
}
