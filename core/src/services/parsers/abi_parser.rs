use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::Read};
use tiny_keccak::{Hasher, Keccak};
use tracing::{debug, instrument};

use crate::{error::ServiceError, model::element::Element};

#[derive(Debug, Deserialize, Serialize)]
struct Input {
    indexed: bool,
    name: String,
    r#type: String,
}

#[instrument]
fn read_file_contents(abi_path: std::path::PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(abi_path)?;
    debug!("file open");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    debug!("file read");
    Ok(contents)
}

#[instrument]
pub async fn parse_abi(
    abi_path: std::path::PathBuf,
    contract_address: &String,
) -> Result<(), serde_json::Error> {
    let contents = match read_file_contents(abi_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Error file reading : {:?}", e),
    };

    let parsed: Value = serde_json::from_str(&contents)?;
    let abi = match parsed.is_object() {
        true => parsed.as_object().unwrap()["abi"].as_array().unwrap(),
        false => parsed.as_array().unwrap(),
    };

    for element in abi
        .iter()
        .filter(|e| e.get("type") != Some(&Value::String("constructor".to_string())))
    {
        let is_param = &element.as_object().unwrap()["inputs"]
            .as_array()
            .unwrap()
            .len()
            > &0;
        let elem_type = element.get("type").unwrap().as_str().unwrap();
        let mut name = element.as_object().unwrap()["name"]
            .as_str()
            .unwrap()
            .to_owned();
        if is_param {
            let mut inputs = String::new();
            inputs.push_str("(");
            for input in element.as_object().unwrap()["inputs"]
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
                json: serde_json::to_string(element).unwrap(),
                signature: generate_signature(name).unwrap(),
                contract_address: contract_address.to_string(),
            };
            save_element(elem_type, new_element).await?;
        } else {
            let new_element = Element {
                name: name.to_string(),
                json: serde_json::to_string(element).unwrap(),
                signature: generate_signature(name).unwrap(),
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
            match crate::model::event::create(&crate::model::event::Event {
                element: new_element,
            })
            .await
            {
                Ok(_) => (),
                Err(error) => panic!("Error saving event : {:?}", error),
            };
        }
        "function" => {
            match crate::model::function::create(&crate::model::function::Function {
                element: new_element,
            })
            .await
            {
                Ok(_) => (),
                Err(error) => panic!("Error saving function : {:?}", error),
            };
        }
        "error" => {
            match crate::model::error::create(&crate::model::error::Error {
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

fn generate_signature(human_readable_signature: String) -> Result<String, ServiceError> {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(human_readable_signature.as_bytes());
    keccak.finalize(&mut output);

    // Concat bytes array
    let mut signature_str = String::new();
    for byte in output.iter() {
        signature_str.push_str(&format!("{:02x}", byte));
    }

    Ok(signature_str)
}
