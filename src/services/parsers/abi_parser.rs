use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Serialize)]
struct Event {
    anonymous: bool,
    inputs: Vec<Input>,
    name: String,
    r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormatedEvent {
    pub name: String,
    pub str: Option<String>,
    pub json: Option<String>,
}

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

pub fn parse_abi(abi_path: std::path::PathBuf) -> Result<Vec<FormatedEvent>, serde_json::Error> {
    let contents = match read_file_contents(abi_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Error file reading : {:?}", e),
    };

    let parsed: Value = serde_json::from_str(&contents)?;
    let events = parsed.as_array().unwrap();
    let mut vec_of_events: Vec<FormatedEvent> = Vec::new();

    for event in events
        .iter()
        .filter(|e| e.get("type") == Some(&Value::String("event".to_string())))
    {
        let is_param = &event.as_object().unwrap()["inputs"]
            .as_array()
            .unwrap()
            .len()
            > &0;
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
            let new_event = FormatedEvent {
                name: event.as_object().unwrap()["name"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                str: Some(inputs),
                json: Some(serde_json::to_string(event).unwrap()),
            };
            vec_of_events.push(new_event);
        } else {
            let new_event = FormatedEvent {
                name: event.as_object().unwrap()["name"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                str: None,
                json: None,
            };
            vec_of_events.push(new_event);
        }
    }

    Ok(vec_of_events)
}
