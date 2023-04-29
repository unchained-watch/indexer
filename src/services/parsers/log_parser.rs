use crate::services::model::data::create;
use crate::services::model::events::Event;
use crate::{error::ServiceError, services::model::events::find_by_signature};
use num::bigint::BigUint;
use num::traits::cast::FromPrimitive;
use num::traits::identities::Zero;
use rustc_hex::FromHex;
use serde_json::Value;
use web3::types::U256;

pub async fn parse_data_bytes(data: &String, topic: &String) -> Result<(), ServiceError> {
    let events = find_by_signature(topic).await?;
    let event = events.first().unwrap();
    println!("========: {:?}",event);
    let _ = event_data_decoder(event, data);

    Ok(())
}

async fn event_data_decoder(event: &Event, data: &String) -> Result<(), ServiceError> {
    let event_abi: Value = serde_json::from_str(event.json.as_str()).unwrap();
    let split = slice_string(data, 64);
    let mut input_index = 0;
    if split.len() == 0 {
        return Ok(());
    }

    for input_param in event_abi["inputs"].as_array().unwrap().iter() {
        let format_value = match input_param["type"].as_str().unwrap() {
            "address" => parse_h160(&split[input_index]),
            "uint256" => hex_string_to_u256(&split[input_index]),
            _ => panic!("Invalid type"),
        };

        let contract_address = &event.contract_address;
        create( event.id.clone().unwrap(),input_param["name"].to_string(),format_value.unwrap(), contract_address.to_string()).await?;
        input_index = input_index + 1;
    }

    Ok(())
}

fn hex_string_to_u256(s: &str) -> Option<String> {
    let bytes: Vec<u8> = match s.from_hex() {
        Ok(b) => b,
        Err(_) => return None,
    };
    let mut result = BigUint::zero();
    for byte in bytes {
        result *= BigUint::from_u8(16).unwrap();
        result += BigUint::from_u8(byte).unwrap();
    }

    Some(U256::from_big_endian(&result.to_bytes_be()).to_string())
}

fn parse_h160(s: &str) -> Option<String> {
    let bytes: Vec<u8> = match str::from_hex(s) {
        Ok(b) => b,
        Err(_) => return None,
    };

    let mut h160 = [0u8; 20];
    h160.copy_from_slice(&bytes[bytes.len() - 20..]);
    Some(web3::types::H160(h160).to_string())
}

fn slice_string(s: &str, chunk_size: usize) -> Vec<String> {
    s.to_string()
        .chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
}
