use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use web3::types::Log;

use crate::db::get_instance_db;
use crate::services::parsers::parse_data_bytes;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub address: String,
    pub topics: String,
    pub data: String,
    pub block_hash: String,
    pub block_number: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub log_index: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create(transaction: Log) -> Result<(), surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut serialized = String::new();
    serialized.push_str(&hex::encode(&transaction.data.0));

    let mut topics = String::new();

    for h256 in &transaction.topics {
        topics.push_str(&format!("{:x}", h256));
    }

    let transaction_hash = transaction.transaction_hash.unwrap().to_string();
    let transactions = find_by_transaction_hash(&transaction_hash).await?;

    if transactions.len() == 0 {
        match parse_data_bytes(
            &serialized,
            &format!("{:x}", transaction.topics[0]),
            &transaction_hash,
        )
        .await
        {
            Ok(()) => (),
            Err(e) => panic!("{}", e),
        };
        let new_transaction = Transaction {
            address: transaction.address.to_string(),
            topics,
            data: serialized,
            block_hash: transaction.block_hash.unwrap().to_string(),
            block_number: transaction.block_number.unwrap().to_string(),
            transaction_hash,
            transaction_index: transaction.transaction_index.unwrap().to_string(),
            log_index: transaction.log_index.unwrap().to_string(),
        };
        let _: Record = db.create("logs").content(new_transaction).await?;
    }
    Ok(())
}

pub async fn find_by_transaction_hash(
    transaction_hash: &String,
) -> Result<Vec<Transaction>, surrealdb::Error> {
    let db = get_instance_db().await.unwrap();

    let mut result = db
        .query("SELECT * FROM logs WHERE transaction_hash = $transaction_hash")
        .bind(("transaction_hash", transaction_hash.to_string()))
        .await?;

    let transactions: Vec<Transaction> = result.take(0)?;

    Ok(transactions)
}
