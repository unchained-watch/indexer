use super::{address::Address, element::Element};
use crate::db::get_instance_db;
use bson::doc;
use mongodb::options::UpdateOptions;
use serde::{Deserialize, Serialize};
use tracing::debug;
use web3::futures::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub element: Element,
}

pub async fn create(event: &Event) -> Result<(), mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();

    let collection = db.collection::<Event>("events");

    let serialized_event = bson::to_bson(event)?;
    let event_document = serialized_event.as_document().unwrap();

    collection
        .update_one(
            doc! {"element.signature": event.element.signature.to_string()},
            doc! {"$set": event_document.to_owned()},
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;

    Ok(())
}

pub async fn find_by_signature(signature: &String) -> Result<Vec<Event>, mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Event>("events");

    let events = collection
        .find(doc! { "signature": signature.to_string() }, None)
        .await?
        .try_collect()
        .await?;

    Ok(events)
}

pub async fn find_by_contract_addresses(
    addresses: Vec<String>,
) -> Result<Vec<String>, mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Address>("addresses");

    let addresses: Vec<Address> = collection
        .find(doc! {"address": {"$in": addresses}}, None)
        .await?
        .try_collect()
        .await?;

    debug!("Addresses {:?}", addresses);

    Ok(addresses
        .iter()
        .map(|event| event.address.to_string())
        .collect())
}

pub async fn find_by_contract_address(
    address: String,
) -> Result<Vec<Event>, mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Event>("events");

    let events = collection
        .find(doc! { "address": address }, None)
        .await?
        .try_collect()
        .await?;

    Ok(events)
}
