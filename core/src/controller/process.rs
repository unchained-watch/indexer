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

    let mut signatures: Vec<Event> = vec![];
    for event_str in parse_abi(abi_path).unwrap().iter() {
        let mut signature_event = String::new();
        signature_event.push_str(&event_str.name);

        if let Some(value) = &event_str.str {
            signature_event.push_str(&value);
        } else {
            signature_event.push_str("()");
        }

        let mut keccak = Keccak::v256();
        let mut output = [0u8; 32];
        keccak.update(signature_event.as_bytes());
        keccak.finalize(&mut output);

        // Concat bytes array
        let mut signature_str = String::new();
        for byte in output.iter() {
            signature_str.push_str(&format!("{:02x}", byte));
        }
        let event = Event {
            id: None,
            name: signature_event,
            signature: signature_str,
            json: event_str.json.clone().unwrap(),
            contract_address: contract_address.to_string(),
        };

        match create(&event).await {
            Ok(_) => (),
            Err(error) => panic!("Error when saving data {}", error),
        };

        signatures.push(event);
    }

    listeners::get_past_events(&contract_address, &signatures, &block_number).await?;
    Ok(())
}

pub async fn get_abi(
    contract_address: String,
    abi_path: std::path::PathBuf,
) -> Result<(), web3::Error> {
    let mut signatures: Vec<Event> = vec![];
    for event_str in parse_abi(abi_path).unwrap().iter() {
        let mut signature_event = String::new();
        signature_event.push_str(&event_str.name);

        if let Some(value) = &event_str.str {
            signature_event.push_str(&value);
        } else {
            signature_event.push_str("()");
        }

        let mut keccak = Keccak::v256();
        let mut output = [0u8; 32];
        keccak.update(signature_event.as_bytes());
        keccak.finalize(&mut output);

        // Concat bytes array
        let mut signature_str = String::new();
        for byte in output.iter() {
            signature_str.push_str(&format!("{:02x}", byte));
        }
        let event = Event {
            id: None,
            name: signature_event,
            signature: signature_str,
            json: event_str.json.clone().unwrap(),
            contract_address: contract_address.to_string(),
        };

        match create(&event).await {
            Ok(_) => (),
            Err(error) => panic!("Error when saving data {}", error),
        };

        signatures.push(event);
    }
    Ok(())
}

pub async fn get_realtime_block() -> Result<(), ServiceError> {
    listeners::get_realtime_events().await?;
    Ok(())
}
