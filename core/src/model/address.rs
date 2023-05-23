use crate::db::get_instance_db;
use bson::doc;
use mongodb::options::UpdateOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AddressType {
    CONTRACT,
    WALLET,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub address_type: AddressType,
    pub address: String,
}

pub async fn create(address: &Address) -> Result<(), mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Address>("addresses");

    let serialized_address = bson::to_bson(address)?;
    let address_document = serialized_address.as_document().unwrap();

    collection
        .update_one(
            doc! {"address": address.address.to_string()},
            doc! {"$set": address_document.to_owned()},
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;

    Ok(())
}
