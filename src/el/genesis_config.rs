use anyhow::{Context, Result};
use std::collections::HashMap;

use super::config::Allocation;

#[derive(Default)]
pub struct GenesisConfig {
    pub preset_base: String,
    pub chain_id: u32,
    pub deposit_contract_address: String,
    pub mnemonic: String,
    pub el_premine: std::collections::HashMap<String, String>,
    pub el_premine_addrs: std::collections::HashMap<String, Allocation>,
    pub additional_preloaded_contracts: std::collections::HashMap<String, Allocation>,
    pub genesis_timestamp: u64,
    pub genesis_delay: u64,
    pub genesis_gaslimit: Option<u64>,
    pub slot_duration_in_seconds: u64,
    pub electra_fork_epoch: Option<u64>,
    pub eof_activation_epoch: Option<u64>,
}

#[derive(Default)]
pub struct GenesisConfigBuilder {
    preset_base: Option<String>,
    chain_id: Option<u32>,
    deposit_contract_address: Option<String>,
    mnemonic: Option<String>,
    el_premine: HashMap<String, String>,
    el_premine_addrs: HashMap<String, Allocation>,
    additional_preloaded_contracts: HashMap<String, Allocation>,
    genesis_timestamp: Option<u64>,
    genesis_delay: Option<u64>,
    genesis_gaslimit: Option<u64>,
    slot_duration_in_seconds: Option<u64>,
    electra_fork_epoch: Option<u64>,
    eof_activation_epoch: Option<u64>,
}

impl GenesisConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_hashmap(mut self, config_map: HashMap<String, String>) -> Result<Self> {
        if let Some(preset_base) = config_map.get("PRESET_BASE") {
            self.preset_base = Some(preset_base.clone());
        }
        if let Some(chain_id) = config_map.get("CHAIN_ID") {
            self.chain_id = chain_id.parse().ok();
        }
        if let Some(deposit_contract_address) = config_map.get("DEPOSIT_CONTRACT_ADDRESS") {
            self.deposit_contract_address = Some(deposit_contract_address.clone());
        }
        if let Some(mnemonic) = config_map.get("EL_AND_CL_MNEMONIC") {
            self.mnemonic = Some(mnemonic.clone());
        }
        if let Some(el_premine_addrs) = config_map.get("EL_PREMINE_ADDRS") {
            self.el_premine_addrs = parse_allocation(&el_premine_addrs)?;
        }

        if let Some(additional_preloaded_contracts) =
            config_map.get("ADDITIONAL_PRELOADED_CONTRACTS")
        {
            self.additional_preloaded_contracts =
                parse_allocation(&additional_preloaded_contracts)?;
        }

        if let Some(genesis_timestamp) = config_map.get("GENESIS_TIMESTAMP") {
            self.genesis_timestamp = genesis_timestamp.parse().ok();
        }
        if let Some(genesis_delay) = config_map.get("GENESIS_DELAY") {
            self.genesis_delay = genesis_delay.parse().ok();
        }
        if let Some(genesis_gaslimit) = config_map.get("GENESIS_GASLIMIT") {
            self.genesis_gaslimit = genesis_gaslimit.parse().ok();
        }
        if let Some(slot_duration_in_seconds) = config_map.get("SLOT_DURATION_IN_SECONDS") {
            self.slot_duration_in_seconds = slot_duration_in_seconds.parse().ok();
        }
        if let Some(electra_fork_epoch) = config_map.get("ELECTRA_FORK_EPOCH") {
            self.electra_fork_epoch = electra_fork_epoch.parse().ok();
        }
        if let Some(eof_activation_epoch) = config_map.get("EOF_ACTIVATION_EPOCH") {
            self.eof_activation_epoch = eof_activation_epoch.parse().ok();
        }
        Ok(self)
    }

    pub fn preset_base(mut self, preset_base: String) -> Self {
        self.preset_base = Some(preset_base);
        self
    }

    pub fn chain_id(mut self, chain_id: u32) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    pub fn deposit_contract_address(mut self, deposit_contract_address: String) -> Self {
        self.deposit_contract_address = Some(deposit_contract_address);
        self
    }

    pub fn mnemonic(mut self, mnemonic: String) -> Self {
        self.mnemonic = Some(mnemonic);
        self
    }

    pub fn el_premine(mut self, key: String, value: String) -> Self {
        self.el_premine.insert(key, value);
        self
    }

    pub fn el_premine_addrs(mut self, key: String, value: Allocation) -> Self {
        self.el_premine_addrs.insert(key, value);
        self
    }

    pub fn additional_preloaded_contracts(mut self, key: String, value: Allocation) -> Self {
        self.additional_preloaded_contracts.insert(key, value);
        self
    }

    pub fn genesis_timestamp(mut self, genesis_timestamp: u64) -> Self {
        self.genesis_timestamp = Some(genesis_timestamp);
        self
    }

    pub fn genesis_delay(mut self, genesis_delay: u64) -> Self {
        self.genesis_delay = Some(genesis_delay);
        self
    }

    pub fn genesis_gaslimit(mut self, genesis_gaslimit: Option<u64>) -> Self {
        self.genesis_gaslimit = genesis_gaslimit;
        self
    }

    pub fn slot_duration_in_seconds(mut self, slot_duration_in_seconds: u64) -> Self {
        self.slot_duration_in_seconds = Some(slot_duration_in_seconds);
        self
    }

    pub fn electra_fork_epoch(mut self, electra_fork_epoch: Option<u64>) -> Self {
        self.electra_fork_epoch = electra_fork_epoch;
        self
    }

    pub fn eof_activation_epoch(mut self, eof_activation_epoch: Option<u64>) -> Self {
        self.eof_activation_epoch = eof_activation_epoch;
        self
    }

    pub fn with_default_premine(mut self) -> Self {
        self.el_premine
            .insert("m/44'/60'/0'/0/0".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/1".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/2".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/3".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/4".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/5".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/6".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/7".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/8".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/9".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/10".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/11".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/12".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/13".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/14".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/15".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/16".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/17".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/18".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/19".to_string(), "1000000000ETH".to_string());
        self.el_premine
            .insert("m/44'/60'/0'/0/20".to_string(), "1000000000ETH".to_string());
        self
    }

    pub fn build(self) -> GenesisConfig {
        GenesisConfig {
            preset_base: self.preset_base.expect("preset_base is required"),
            chain_id: self.chain_id.expect("chain_id is required"),
            deposit_contract_address: self
                .deposit_contract_address
                .expect("deposit_contract_address is required"),
            mnemonic: self.mnemonic.expect("mnemonic is required"),
            el_premine: self.el_premine,
            el_premine_addrs: self.el_premine_addrs,
            additional_preloaded_contracts: self.additional_preloaded_contracts,
            genesis_timestamp: self
                .genesis_timestamp
                .expect("genesis_timestamp is required"),
            genesis_delay: self.genesis_delay.expect("genesis_delay is required"),
            genesis_gaslimit: self.genesis_gaslimit,
            slot_duration_in_seconds: self
                .slot_duration_in_seconds
                .expect("slot_duration_in_seconds is required"),
            electra_fork_epoch: self.electra_fork_epoch,
            eof_activation_epoch: self.eof_activation_epoch,
        }
    }
}

fn parse_allocation<'a>(str: &'a str) -> Result<HashMap<String, Allocation>> {
    serde_json::from_str(&str).context("Can't parse string as allocation struct")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_parsing() {
        let str = "{\"0x123463a4B065722E99115D6c222f267d9cABb524\": {\"balance\": \"1ETH\",\"code\": \"0x123465\",\"storage\": {},\"nonce\": 0,\"secretKey\": \"0x\"}}";

        let result = parse_allocation(str);
        assert!(result.is_ok());
        let map = result.unwrap();

        let address = "0x123463a4B065722E99115D6c222f267d9cABb524";
        let balance = "1ETH";
        let code = "0x123465";
        let nonce = 0;
        let secret_key = "0x";

        assert_eq!(map[address].balance, balance);
        assert_eq!(map[address].code, Some(code.into()));
        assert_eq!(map[address].nonce, Some(nonce));
        assert_eq!(map[address].secret_key, Some(secret_key.into()));

        // let storage = &map[address].storage;
        // assert!(storage.unwrap().is_empty());
    }
}
