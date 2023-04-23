use serde::{Deserialize, Serialize};

use crate::db::get_instance_db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub signature: String,
    pub json: String,
}

pub async fn create(event: &Event) -> Result<Event, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();
    let created: Event = match db.create("events").content(event).await {
        Ok(value) => value,
        Err(e) => panic!("{}", e),
    };

    Ok(created)
}

pub async fn find_by_signature(signature: &String) -> Result<Event, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut all_event = db
        .query("SELECT * FROM events")
        .await?;

    println!("{:?}",all_event);

    let mut result = db
        .query("SELECT * FROM events WHERE events.signature = $signature")
        .bind(("signature", signature))
        .await?;

    let event: Option<Event> = result.take(0)?;
    Ok(event.unwrap())
}
