use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    Client, Database,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigDatabase {
    url: String,
    name: String,
}

impl ConfigDatabase {
    pub fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

        ConfigDatabase { url, name }
    }
}

#[tracing::instrument]
pub async fn get_instance_db() -> Result<Database, Error> {
    let db_config = ConfigDatabase::new();

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&db_config.url, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let db = client.database(&db_config.name);

    Ok(db)
}
