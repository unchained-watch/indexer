use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tracing::error;

use super::element::Element;

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    #[allow(dead_code)]
    pub id: Option<Thing>,
    pub element: Element,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create(function: &Function) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let functions = find_by_name_and_contract_address(
        &function.element.name,
        &function.element.contract_address,
    )
    .await?;
    if functions.len() == 0 {
        let _: Record = match db.create("function").content(function).await {
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
) -> Result<Vec<Function>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query(
            "SELECT * FROM function WHERE element.name = $name AND element.contract_address = $contract_address",
        )
        .bind(("name", name.to_string()))
        .bind(("contract_address", contract_address.to_string()))
        .await?;

    let function: Vec<Function> = result.take(0)?;

    Ok(function)
}
