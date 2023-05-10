use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[allow(dead_code)]
    pub id: Option<Thing>,
    pub element: crate::common::Element,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create(event: &Event) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let events =
        find_by_signature_and_contract_address(&event.element.signature, &event.element.contract_address).await?;
    if events.len() == 0 {
        let _: Record = match db.create("events").content(event).await {
            Ok(id) => id,
            Err(error) => {
                println!("{:?}", error);
                panic!("{:?}", error)
            }
        };
    }
    Ok(())
}

pub async fn find_by_signature_and_contract_address(
    signature: &String,
    contract_address: &String,
) -> Result<Vec<Event>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM events WHERE element.signature = $signature AND element.contract_address = $contract_address")
        .bind(("signature", signature.to_string()))
        .bind(("contract_address", contract_address.to_string()))
        .await?;

    let event: Vec<Event> = result.take(0)?;

    Ok(event)
}

pub async fn find_by_signature(signature: &String) -> Result<Vec<Event>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM events WHERE element.signature = $signature")
        .bind(("signature", signature.to_string()))
        .await?;

    let event: Vec<Event> = result.take(0)?;

    Ok(event)
}

pub async fn find_by_contract_addresses(
    addresses: Vec<String>,
) -> Result<Vec<String>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT contract_address FROM events WHERE element.contract_address CONTAINSANY $addresses")
        .bind(("addresses", addresses))
        .await?;

    let contract_addresses: Vec<String> = result.take("contract_address")?;

    Ok(contract_addresses)
}

pub async fn find_by_contract_address(
    address: String,
) -> Result<Vec<Event>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM events WHERE element.contract_address = $address")
        .bind(("address", address))
        .await?;

    let events: Vec<Event> = result.take(0)?;

    Ok(events)
}
