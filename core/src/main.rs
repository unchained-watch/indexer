mod controller;
mod db;
mod error;
mod model;
mod services;
use clap::Parser;
use error::ServiceError;

use crate::{
    db::get_instance_db,
    model::addresses::{Address, AddressType},
};

#[derive(Parser)]
struct Cli {
    contract_address: String,
    tx_hash: String,
    abi_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let args = Cli::parse();

    println!("Using contract_address: {}", args.contract_address);
    println!("Using tx_hash: {}", args.tx_hash);
    println!("Using abi_path: {}", args.abi_path.to_str().unwrap());

    //Init index in database
    let db = get_instance_db().await.unwrap();
    match db
    .query("DEFINE INDEX eventContractAndSignatureIndex ON TABLE events COLUMNS signature, contract_address UNIQUE;")
    .await{
        Ok(value) => println!("Indexes : {:?}",value),
        Err(error)=>panic!("Error : {:?}",error)
    };
    match db
        .query("DEFINE INDEX transactionIndex ON TABLE logs COLUMNS transaction_hash UNIQUE;")
        .await
    {
        Ok(value) => println!("Indexes : {:?}", value),
        Err(error) => panic!("Error : {:?}", error),
    };

    match db
        .query("DEFINE INDEX datasIndex ON TABLE datas COLUMNS tx,name,value UNIQUE;")
        .await
    {
        Ok(value) => println!("Indexes : {:?}", value),
        Err(error) => panic!("Error : {:?}", error),
    };

    let address_to_watch = Address {
        id: None,
        address_type: AddressType::CONTRACT,
        address: args.contract_address,
    };

    model::addresses::create(&address_to_watch).await?;
    controller::get_history(address_to_watch.address, &args.tx_hash, args.abi_path).await?;
    controller::get_realtime_block().await?;
    Ok(())
}
