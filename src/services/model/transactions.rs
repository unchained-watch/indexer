use mongodb::bson::{doc, Document};
use web3::types::Log;

use crate::db::get_instance_db;
use crate::services::parsers::parse_data_bytes;

pub async fn create(transaction: Log) -> Result<(), mongodb::error::Error> {
    let db = get_instance_db().await.unwrap();
    let collection = db.collection::<Document>("logs");

    let mut serialized = String::new();
    serialized.push_str(&hex::encode(&transaction.data.0));

    let mut topics = String::new();

    for h256 in &transaction.topics {
        topics.push_str(&format!("{:x}", h256));
    }

    match parse_data_bytes(&serialized, &format!("{:x}", transaction.topics[0])).await {
        Ok(()) => (),
        Err(e) => panic!("{}", e),
    };

    let docs = vec![doc! {
        "address": transaction.address.to_string(),
        "topics": topics,
        "data": serialized,
        "block_hash":transaction.block_hash.unwrap().to_string(),
        "block_number": transaction.block_number.unwrap().to_string(),
        "transaction_hash": transaction.transaction_hash.unwrap().to_string(),
        "transaction_index": transaction.transaction_index.unwrap().to_string(),
        "log_index": transaction.log_index.unwrap().to_string(),
    }];

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;
    Ok(())
}
