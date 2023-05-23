use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub tx: String,
    pub name: String,
    pub contract_address: String,
    pub value: String,
}

pub async fn create(
    tx: String,
    name: String,
    value: String,
    contract_address: String,
) -> Result<(), mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Data>("data");

    let data = Data {
        tx,
        name,
        value,
        contract_address,
    };

    collection.insert_one(data, None).await?;

    Ok(())
}
