use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tracing::error;

use super::element::Element;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[allow(dead_code)]
    pub id: Option<Thing>,
    pub element: Element,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create(error: &Error) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let errors =
        find_by_name_and_contract_address(&error.element.name, &error.element.contract_address)
            .await?;
    if errors.len() == 0 {
        let _: Record = match db.create("error").content(error).await {
            Ok(id) => id,
            Err(e) => {
                error!("{:?}", e);
                panic!("{:?}", e)
            }
        };
    }
    Ok(())
}

pub async fn find_by_name_and_contract_address(
    name: &String,
    contract_address: &String,
) -> Result<Vec<Error>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM error WHERE element.name = $name AND element.contract_address = $contract_address")
        .bind(("name", name.to_string()))
        .bind(("contract_address", contract_address.to_string()))
        .await?;

    let error: Vec<Error> = result.take(0)?;

    Ok(error)
}
