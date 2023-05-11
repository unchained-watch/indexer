use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub tx: String,
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
    tx: String,
    name: String,
    value: String,
    contract_address: String,
) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let data = Data {
        tx,
        name,
        value,
        contract_address,
    };
    let _: Record = match db.create("data").content(data).await {
        Ok(id) => id,
        Err(error) => panic!("{:?}", error),
    };

    Ok(())
}
