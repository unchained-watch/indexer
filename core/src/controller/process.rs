use tiny_keccak::{Hasher, Keccak};

use services::listeners;

use crate::{
    error::ServiceError,
    model::events::{create, Event},
    services::{self, parsers::parse_abi},
};

pub async fn get_history(
    contract_address: String,
    tx_hash: &String,
    abi_path: std::path::PathBuf,
) -> Result<(), web3::Error> {
    let block_number_result = listeners::get_first_block_from_tx_hash(&tx_hash).await;
    let contract_address = &contract_address;
    let block_number = match block_number_result {
        Ok(block_number) => block_number,
        Err(error) => panic!("Error : {:?}", error),
    };

   
   // listeners::get_past_events(&contract_address, &signatures, &block_number).await?;
    Ok(())
}

pub async fn get_abi(
    contract_address: String,
    abi_path: std::path::PathBuf,
) -> Result<(), web3::Error> {

    
    parse_abi(abi_path, &contract_address).await?;
       
    Ok(())
}

pub async fn get_realtime_block() -> Result<(), ServiceError> {
    listeners::get_realtime_events().await?;
    Ok(())
}
