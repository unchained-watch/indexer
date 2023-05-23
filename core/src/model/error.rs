use crate::db::get_instance_db;
use bson::doc;
use mongodb::options::UpdateOptions;
use serde::{Deserialize, Serialize};

use super::element::Element;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub element: Element,
}

pub async fn create(error: &Error) -> Result<(), mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Error>("errors");

    let serialized_error = bson::to_bson(error)?;
    let error_document = serialized_error.as_document().unwrap();

    collection
        .update_one(
            doc! {"element.signature": error.element.signature.to_string()},
            doc! {"$set": error_document.to_owned()},
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;
    Ok(())
}
