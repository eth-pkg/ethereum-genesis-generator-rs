use std::collections::HashMap;

use super::config::Allocation;

pub struct GenesisConfig {
    pub chain_id: u32,
    pub deposit_contract_address: String,
    pub genesis_gaslimit: Option<u64>,
    pub genesis_timestamp: u64,
    pub mnemonic: String,
    pub el_premine: std::collections::HashMap<String, String>,
    pub el_premine_addrs: std::collections::HashMap<String, Allocation>,
    pub additional_preloaded_contracts: std::collections::HashMap<String, Allocation>,
    pub electra_fork_epoch: Option<u64>,
    pub genesis_delay: u64,
    pub preset_base: String,
    pub slot_duration_in_seconds: u64,
    pub eof_activation_epoch: Option<u64>,
}


impl TryFrom<HashMap<String, String>> for GenesisConfig {
    type Error = String;

    fn try_from(context: HashMap<String, String>) -> Result<Self, Self::Error> {
        if let Some(chain_id) = context.get("CHAIN_ID") {
            let chain_id = chain_id.parse().unwrap();
            let deposit_contract_address = context
                .get("DEPOSIT_CONTRACT_ADDRESS")
                .unwrap_or(&"".to_string())
                .clone();
            let genesis_gaslimit = context
                .get("GENESIS_GASLIMIT")
                .map(|num| num.parse().unwrap());
            let genesis_timestamp = context
                .get("GENESIS_TIMESTAMP")
                .map(|num| num.parse().unwrap())
                .unwrap_or(0);
            let mnemonic = context.get("MNEMONIC").unwrap_or(&"".to_string()).clone();
            // todo
            let el_premine = HashMap::new();
            // todo 
            let el_premine_addrs = HashMap::new();
            // todo 
            let additional_preloaded_contracts = HashMap::new();
            let electra_fork_epoch = context
                .get("ELECTRA_FORK_EPOCH")
                .map(|num| num.parse().unwrap());
            let genesis_delay: u64 = context
                .get("GENESIS_DELAY")
                .map(|num| num.parse().unwrap())
                .unwrap_or(60);
            let preset_base: String = context
                .get("PRESET_BASE")
                .unwrap_or(&"".to_string())
                .clone();
            let slot_duration_in_seconds = context
                .get("DEPOSIT_CONTRACT_ADDRESS")
                .map(|num| num.parse().unwrap())
                .unwrap_or(12);
            let eof_activation_epoch = context
                .get("EOF_ACTIVATION_EPOCH")
                .map(|num| num.parse().unwrap());
            let genesis_config = GenesisConfig {
                chain_id,
                deposit_contract_address,
                genesis_gaslimit,
                genesis_timestamp,
                mnemonic,
                el_premine,
                el_premine_addrs,
                additional_preloaded_contracts,
                electra_fork_epoch,
                genesis_delay,
                preset_base,
                slot_duration_in_seconds,
                eof_activation_epoch,
            };
            return Ok(genesis_config);
        }
        Err("Could not parse chain_id from context. Please provide this value!".into())
    }
}
