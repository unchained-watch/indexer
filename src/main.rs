mod controller;
mod db;
mod error;
mod services;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FormatedSignature {
    pub name: String,
    pub signature: String,
    pub json: String,
}

#[derive(Parser)]
struct Cli {
    contract_address: String,
    tx_hash: String,
    abi_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    let args = Cli::parse();

    println!("Using contract_address: {}", args.contract_address);
    println!("Using tx_hash: {}", args.tx_hash);
    println!("Using abi_path: {}", args.abi_path.to_str().unwrap());

    controller::get_history(args.contract_address, &args.tx_hash, args.abi_path).await?;

    Ok(())
}
