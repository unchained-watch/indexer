use hex::FromHex;
use std::convert::TryInto;
use std::env;
use std::str::FromStr;
use tokio::task::JoinHandle;
use tracing::debug;
use web3::futures::StreamExt;
use web3::transports::WebSocket;
use web3::types::{BlockNumber, Log, TransactionId};
use web3::types::{FilterBuilder, H256, U64};
use web3::Web3;

use crate::error::ServiceError;
use crate::model::event::{find_by_contract_address, find_by_contract_addresses};

pub async fn get_first_block_from_tx_hash(
    tx_hash: &String,
) -> Result<U64, Box<dyn std::error::Error>> {
    // Initialize connexion with web3 crate in websocket mod
    let websocket = WebSocket::new(&env::var("ANVIL_RPC_URL_WS").unwrap()).await?;

    // Handle success case
    let web3 = Web3::new(websocket);
    let receipt = web3
        .eth()
        .transaction_receipt(H256::from_str(tx_hash).unwrap())
        .await;
    if let Ok(Some(receipt)) = receipt {
        let block_number = receipt
            .block_number
            .expect("Block number missing in receipt");

        debug!("block_number : {:?}", block_number);
        return Ok(block_number);
    }

    Err("Something went wrong!".to_string().into())
}

pub async fn get_past_events(
    contract_address: &String,
    block_number: &U64,
) -> Result<(), ServiceError> {
    let websocket = WebSocket::new(&env::var("ANVIL_RPC_URL_WS").unwrap()).await?;
    let web3 = Web3::new(websocket);

    let to = web3.eth().block_number().await?;
    let diff = to - block_number;
    let mut euclide: Option<i64> = None;
    let mut rest: Option<i64> = None;
    if diff.as_u64() as i64 > 10000 {
        euclide = Some(diff.as_u64() as i64 / 10000);
        rest = Some(diff.as_u64() as i64 % 1000);
    }

    debug!("Parse event from : {:?}", block_number);
    debug!("Parse event to : {:?}", to);
    debug!("Parse event contractAddress : {:?}", contract_address);

    let mut tasks: Vec<JoinHandle<()>> = vec![];
    let events = find_by_contract_address(contract_address.to_string()).await?;

    for event in events.iter() {
        debug!("signature : {:?}", &event.element);

        let hex = Vec::from_hex(&event.element.signature[..]).expect("invalid hex string");
        match euclide {
            Some(n) => {
                let mut from = block_number.to_owned();
                let mut to = block_number.to_owned();

                for _ in std::iter::repeat(()).take(n.try_into().unwrap()) {
                    to = to + 10_000;

                    debug!("range block : {:?}", to - from);

                    tasks.push(
                        filter_events(&event.element.contract_address, &from, &to, &hex).await?,
                    );
                    from = from + 10_000;
                }
                if let Some(r) = rest {
                    to = to + r;
                    tasks.push(
                        filter_events(&event.element.contract_address, &from, &to, &hex).await?,
                    );
                }
            }
            None => {
                tasks.push(
                    filter_events(&event.element.contract_address, block_number, &to, &hex).await?,
                );
            }
        }
    }
    for task in tasks {
        match task.await {
            Ok(_) => (),
            Err(e) => panic!("Failed to complete task: {}", e),
        }
    }
    Ok(())
}

pub async fn filter_events(
    contract_address: &String,
    from: &U64,
    to: &U64,
    hex: &Vec<u8>,
) -> Result<JoinHandle<()>, ServiceError> {
    // Initialize connexion with web3 crate in websocket mod
    let websocket = WebSocket::new(&env::var("ANVIL_RPC_URL_WS").unwrap()).await?;
    // Handle success case
    let web3 = Web3::new(websocket);
    let filter = FilterBuilder::default()
        .address(vec![
            web3::types::Address::from_str(contract_address).unwrap()
        ])
        .from_block(from.into())
        .to_block(to.into())
        .topics(
            Some(vec![H256::from_slice(&hex[..]).try_into().unwrap()]),
            None,
            None,
            None,
        )
        .build();

    let logs: Vec<Log> = web3.eth().logs(filter).await.unwrap();

    let task = tokio::spawn(async move {
        for log in logs {
            match crate::model::transaction::create(log).await {
                Ok(_) => (),
                Err(error) => panic!("Error when saving data {}", error),
            };
        }
    });

    Ok(task)
}

pub async fn get_realtime_events() -> Result<(), ServiceError> {
    let websocket = WebSocket::new(&env::var("ANVIL_RPC_URL_WS").unwrap()).await?;
    let web3: Web3<WebSocket> = Web3::new(websocket);
    let mut tasks = vec![];

    let mut sub = web3.eth_subscribe().subscribe_new_heads().await?;

    debug!("real time events subscription id: {:?}", sub.id());

    let task = tokio::spawn(async move {
        loop {
            if let Some(block) = (&mut sub).next().await {
                // specify the block number you want to check
                let u64_block_number = block.unwrap().number.unwrap();
                let block_number = BlockNumber::Number(u64_block_number);

                // get the block information
                let block = web3
                    .eth()
                    .block(web3::types::BlockId::Number(block_number))
                    .await
                    .unwrap();

                let mut contract_found = false;
                let mut addresses = Vec::new();
                for transaction in block.unwrap().transactions {
                    let tx = web3
                        .eth()
                        .transaction(TransactionId::Hash(transaction))
                        .await
                        .unwrap();
                    let tx_data = tx.unwrap();
                    if tx_data.from.is_some() {
                        addresses.push(tx_data.from.unwrap().to_string());
                    }
                    if tx_data.to.is_some() {
                        addresses.push(tx_data.to.unwrap().to_string());
                    }
                }
                match find_by_contract_addresses(addresses).await {
                    Ok(value) => {
                        if value.len() > 0 {
                            contract_found = true;
                        }
                    }
                    Err(error) => panic!("Error : {:?}", error),
                };
                // determine if the contract address is included in the block
                if contract_found {
                    debug!("One of contract is included in block {}", u64_block_number);
                }
            }
        }
    });
    tasks.push(task);

    for task in tasks {
        match task.await {
            Ok(_) => (),
            Err(e) => panic!("Failed to complete task: {}", e),
        }
    }
    Ok(())
}
