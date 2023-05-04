use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[allow(dead_code)]
    pub id: Option<Thing>,
    pub address_type: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create(address: &Address) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let functions = find_by_address(&address.address).await?;
    if functions.len() == 0 {
        let _: Record = match db.create("addresses").content(function).await {
            Ok(id) => id,
            Err(error) => {
                println!("{:?}", error);
                panic!("{:?}", error)
            }
        };
    }
    Ok(())
}

pub async fn find_by_address(address: &String) -> Result<Vec<Address>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM addresses WHERE address = $address")
        .bind(("address", address.to_string()))
        .await?;

    let addresses: Vec<Address> = result.take(0)?;

    Ok(addresses)
}