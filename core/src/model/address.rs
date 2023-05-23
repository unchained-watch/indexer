use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub enum AddressType {
    CONTRACT,
    WALLET,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[allow(dead_code)]
    pub id: Option<Thing>,
    pub address_type: AddressType,
    pub address: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create(address: &Address) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let addresses = find_by_address(&address.address).await?;
    if addresses.len() == 0 {
        let _: Record = match db.create("address").content(address).await {
            Ok(id) => id,
            Err(e) => {
                error!("{:?}", e);
                panic!("{:?}", e)
            }
        };
    }
    Ok(())
}

pub async fn find_by_address(address: &String) -> Result<Vec<Address>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM address WHERE address = $address")
        .bind(("address", address.to_string()))
        .await?;

    let addresses: Vec<Address> = result.take(0)?;

    Ok(addresses)
}
