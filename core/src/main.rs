mod controller;
mod db;
mod error;
mod model;
mod services;
use clap::Parser;
use common::{config::Config, environment::is_development};
use dotenv::dotenv;
use error::ServiceError;
use tracing::{info, Level};
use tracing_subscriber;

use crate::model::address::{Address, AddressType};

#[derive(Parser)]
struct Cli {
    contract_address: String,
    tx_hash: String,
    abi_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let config: Config = Config::new();

    if is_development() {
        dotenv::from_filename(config.dotenv_path).ok();
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    } else {
        dotenv().ok();
        tracing_subscriber::fmt::init();
    }

    let args = Cli::parse();

    info!(contract_address = args.contract_address);
    info!(tx_hash = args.tx_hash);
    info!(abi_path = args.abi_path.to_str().unwrap());

    let address_to_watch = Address {
        id: None,
        address_type: AddressType::CONTRACT,
        address: args.contract_address,
    };

    model::address::create(&address_to_watch).await?;
    controller::get_abi(address_to_watch.address.to_string(), args.abi_path).await?;
    controller::get_history(address_to_watch.address, &args.tx_hash).await?;
    controller::get_realtime_block().await?;
    Ok(())
}
