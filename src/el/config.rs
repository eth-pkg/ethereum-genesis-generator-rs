use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub chain_id: u32,
    pub homestead_block: u32,
    pub eip150_block: u32,
    pub eip155_block: u32,
    pub eip158_block: u32,
    pub byzantium_block: u32,
    pub constantinople_block: u32,
    pub petersburg_block: u32,
    pub istanbul_block: u32,
    pub berlin_block: u32,
    pub london_block: u32,
    pub pre_merge_fork_block: u32,
    pub terminal_total_difficulty: u64,
    pub shanghai_time: u64,
    pub cancun_time: u64,
    pub deposit_contract_address: String,
    #[serde(rename = "pragueEOFTime")]
    pub prague_time: Option<u64>,
    pub prague_eoftime: Option<u64>,
    pub ethash: std::collections::HashMap<String, String>,
}

impl Config {
    pub fn new(chain_id: u32, deposit_contract_address: String) -> Config {
        Config {
            chain_id,
            homestead_block: 0,
            eip150_block: 0,
            eip155_block: 0,
            eip158_block: 0,
            byzantium_block: 0,
            constantinople_block: 0,
            petersburg_block: 0,
            istanbul_block: 0,
            berlin_block: 0,
            london_block: 0,
            pre_merge_fork_block: 0,
            terminal_total_difficulty: 0,
            shanghai_time: 0,
            cancun_time: 0,
            deposit_contract_address,
            prague_time: None,
            prague_eoftime: None,
            ethash: std::collections::HashMap::new(),
        }
    }
}
 


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub balance: String,
    pub code: Option<String>,
    pub storage: Option<std::collections::HashMap<String, String>>,
    pub nonce: Option<u64>,
    pub secret_key: Option<String>,
}
