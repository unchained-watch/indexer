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
