use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::db::get_instance_db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub signature: String,
    pub json: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}


pub async fn create(event: &Event) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let events = find_by_signature(&event.signature).await?;
    if events.len() == 0 {
        let created : Record = db.create("events").content(event).await?;
    }
    Ok(())
}

pub async fn find_by_signature(signature: &String) -> Result<Vec<Event>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM events WHERE signature = $signature")
        .bind(("signature", signature.to_string()))
        .await?;

    let event: Vec<Event> = result.take(0)?;

    Ok(event)
}
