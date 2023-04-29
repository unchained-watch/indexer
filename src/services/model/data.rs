use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub event_id: Thing,
    pub name: String,
    pub contract_address: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create(
    event_id: Thing,
    name: String,
    value: String,
    contract_address: String,
) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let data = Data {
        event_id,
        name,
        value,
        contract_address,
    };
    let created: Record = db.create("datas").content(data).await?;

    Ok(())
}
