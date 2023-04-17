use mongodb::{error::Error, options::ClientOptions, Client, Database};

pub async fn get_instance_db() -> Result<Database, Error> {
    // create a client options
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    // customize client options as needed
    client_options.app_name = Some("indexation".to_string());

    // create a client and connect to the database
    let client = Client::with_options(client_options).unwrap();
    Ok(client.database("bc-index"))
}
