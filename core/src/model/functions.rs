use crate::db::get_instance_db;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
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

pub async fn create(function: &Function) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let functions =
        find_by_name_and_contract_address(&function.name, &function.contract_address).await?;
    if functions.len() == 0 {
        let _: Record = match db.create("functions").content(function).await {
            Ok(id) => id,
            Err(error) => {
                println!("{:?}", error);
                panic!("{:?}", error)
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
            "SELECT * FROM functions WHERE name = $name AND contract_address = $contract_address",
        )
        .bind(("name", name.to_string()))
        .bind(("contract_address", contract_address.to_string()))
        .await?;

    let function: Vec<Function> = result.take(0)?;

    Ok(function)
}
