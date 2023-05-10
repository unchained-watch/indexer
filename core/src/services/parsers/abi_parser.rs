use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::Read};
use tiny_keccak::{Hasher, Keccak};

use crate::{common::Element, error::ServiceError};

#[derive(Debug, Deserialize, Serialize)]
struct Input {
    indexed: bool,
    name: String,
    r#type: String,
}

fn read_file_contents(abi_path: std::path::PathBuf) -> Result<String, std::io::Error> {
    println!("------------- load file -------------");
    println!();
    let mut file = File::open(abi_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!();
    println!("------------- loaded -------------");
    Ok(contents)
}

pub async fn parse_abi(
    abi_path: std::path::PathBuf,
    contract_address: &String,
) -> Result<(), serde_json::Error> {
    let contents = match read_file_contents(abi_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Error file reading : {:?}", e),
    };

    let parsed: Value = serde_json::from_str(&contents)?;
    let events = parsed.as_array().unwrap();

    for event in events
        .iter()
        .filter(|e| e.get("type") != Some(&Value::String("constructor".to_string())))
    {
        let is_param = &event.as_object().unwrap()["inputs"]
            .as_array()
            .unwrap()
            .len()
            > &0;
        let elem_type = event.get("type").unwrap().as_str().unwrap();
        let mut name = event.as_object().unwrap()["name"]
            .as_str()
            .unwrap()
            .to_owned();
        if is_param {
            let mut inputs = String::new();
            inputs.push_str("(");
            for input in event.as_object().unwrap()["inputs"]
                .as_array()
                .unwrap()
                .iter()
            {
                inputs.push_str(input.as_object().unwrap()["type"].as_str().unwrap());
                inputs.push_str(",");
            }
            inputs = inputs.trim_end_matches(',').to_string();
            inputs.push_str(")");
            name.push_str(&inputs);
            let new_element = Element {
                name: name.to_string(),
                json: serde_json::to_string(event).unwrap(),
                signature: generate_signature(&name, Some(inputs)).unwrap(),
                contract_address: contract_address.to_string(),
            };
            save_element(elem_type, new_element).await?;
        } else {
            let new_element = Element {
                name: name.to_string(),
                json: serde_json::to_string(event).unwrap(),
                signature: generate_signature(&name, None).unwrap(),
                contract_address: contract_address.to_string(),
            };
            save_element(elem_type, new_element).await?;
        }
    }

    Ok(())
}

async fn save_element(elem_type: &str, new_element: Element) -> Result<(), serde_json::Error> {
    match elem_type {
        "event" => {
            match crate::model::events::create(&crate::model::events::Event {
                id: None,
                element: new_element,
            })
            .await
            {
                Ok(_) => (),
                Err(error) => panic!("Error saving event : {:?}", error),
            };
        }
        "function" => {
            match crate::model::functions::create(&crate::model::functions::Function {
                id: None,
                element: new_element,
            })
            .await
            {
                Ok(_) => (),
                Err(error) => panic!("Error saving function : {:?}", error),
            };
        }
        "error" => {
            match crate::model::errors::create(&crate::model::errors::Error {
                id: None,
                element: new_element,
            })
            .await
            {
                Ok(_) => (),
                Err(error) => panic!("Error saving error : {:?}", error),
            };
        }
        _ => {}
    };

    Ok(())
}

fn generate_signature(
    name: &String,
    human_readable_signature: Option<String>,
) -> Result<String, ServiceError> {
    let mut signature_event = String::new();
    signature_event.push_str(name);

    if let Some(value) = human_readable_signature {
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

    Ok(signature_str)
}
