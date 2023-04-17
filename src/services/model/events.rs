use bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use crate::db::get_instance_db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "_id")] // use the MongoDB ObjectId as the struct ID
    pub id: Option<ObjectId>,
    pub name: String,
    pub signature: String,
    pub json: String,
}

pub async fn create(event: &Event) -> Result<(), mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Document>("events");

    let docs = vec![doc! {
        "name": &event.name,
        "signature": &event.signature,
        "json": &event.json
    }];

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;
    Ok(())
}

pub async fn find_by_signature(signature: &String) -> Result<Event, mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let filter = doc! { "signature": signature };
    let collection = db.collection::<Event>("events");

    let event = collection
        .find_one(filter, None)
        .await?
        .expect("Missing event document for this signature.");

    Ok(event)
}
