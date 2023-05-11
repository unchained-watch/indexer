use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum AddressType {
    CONTRACT,
    WALLET,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AbiElementType {
    EVENT,
    ERROR,
    FUNCTION,
    NONE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    pub name: String,
    pub signature: String,
    pub json: String,
    pub contract_address: String,
}
