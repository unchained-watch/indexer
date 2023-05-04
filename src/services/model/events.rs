use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[allow(dead_code)]
    pub id: Option<Thing>,
    pub name: String,
    pub signature: String,
    pub json: String,
    pub contract_address: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create(event: &Event) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let events = find_by_signature(&event.signature).await?;
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

pub async fn find_by_signature(signature: &String) -> Result<Vec<Event>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM events WHERE signature = $signature")
        .bind(("signature", signature.to_string()))
        .await?;

    let event: Vec<Event> = result.take(0)?;

    Ok(event)
}

pub async fn find_by_contract_address(addresses: Vec<String>) -> Result<Vec<String>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT contract_address FROM events WHERE contract_address CONTAINSANY $addresses")
        .bind(("addresses",addresses))
        .await?;

    let contract_addresses: Vec<String> = result.take("contract_address")?;

    Ok(contract_addresses)
}
