use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    pub name: String,
    pub signature: String,
    pub json: String,
    pub contract_address: String,
}
