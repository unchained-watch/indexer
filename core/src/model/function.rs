use super::element::Element;
use crate::db::get_instance_db;
use bson::doc;
use mongodb::options::UpdateOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub element: Element,
}

pub async fn create(function: &Function) -> Result<(), mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Function>("functions");

    let serialized_function = bson::to_bson(function)?;
    let function_document = serialized_function.as_document().unwrap();

    collection
        .update_one(
            doc! {"element.signature": function.element.signature.to_string()},
            doc! {"$set": function_document.to_owned()},
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;
    Ok(())
}
