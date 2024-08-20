use ethers::signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;


use super::{config::{Allocation, Config}, genesis_config::GenesisConfig, serializabe_to_file::{Genesis, SerializableToFile}};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BesuGenesisConfig {
    config: Config,
    alloc: std::collections::HashMap<String, Allocation>,
    coinbase: String,
    base_fee_per_gas: String,
    difficulty: String,
    extra_data: String,
    gas_limit: String,
    nonce: String,
    mixhash: String,
    parent_hash: String,
    timestamp: String,
}

impl BesuGenesisConfig {
    fn for_mainnet() -> Self {
        let mainnet_config_path = "../data/mainnet/besu_genesis.json";
        serde_json::from_reader(fs::File::open(mainnet_config_path).unwrap()).unwrap()
    }
    fn for_holesky() -> Self {
        let holesky_config_path = "../data/holesky/besu_genesis.json";
        serde_json::from_reader(fs::File::open(holesky_config_path).unwrap()).unwrap()
    }
    fn for_sepolia() -> Self {
        let sepolia_config_path = "../data/sepolia/besu_genesis.json";
        serde_json::from_reader(fs::File::open(sepolia_config_path).unwrap()).unwrap()
    }
    fn for_devnet(genesis_config: &GenesisConfig) -> Self {
        BesuGenesisConfig {
            config: Config::new(
                genesis_config.chain_id,
                genesis_config.deposit_contract_address.clone(),
            ),
            alloc: std::collections::HashMap::new(),
            coinbase: "0x0000000000000000000000000000000000000000".to_string(),
            base_fee_per_gas: "0x3B9ACA00".to_string(),
            difficulty: "0x01".to_string(),
            extra_data: "".to_string(),
            gas_limit: format!(
                "0x{:x}",
                genesis_config.genesis_gaslimit.unwrap_or(25000000)
            ),
            nonce: "0x1234".to_string(),
            mixhash: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            parent_hash: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            timestamp: genesis_config.genesis_timestamp.to_string(),
        }
    }
}
impl Genesis for BesuGenesisConfig {
    fn create_genesis(genesis_config: &GenesisConfig) -> BesuGenesisConfig {
        let mut besu_genesis_config = match genesis_config.chain_id {
            1 => BesuGenesisConfig::for_mainnet(),
            11155111 => BesuGenesisConfig::for_sepolia(),
            17000 => BesuGenesisConfig::for_holesky(),
            _ => BesuGenesisConfig::for_devnet(&genesis_config)
        };
    
        for (_, value) in &genesis_config.el_premine {
            let wallet: LocalWallet = MnemonicBuilder::<English>::default()
                .phrase(genesis_config.mnemonic.as_str())
                .build()
                .unwrap();
            let weival = value.replace("ETH", "000000000000000000");
            besu_genesis_config.alloc.insert(
                format!("{:?}", wallet.address()),
                Allocation {
                    balance: weival,
                    code: None,
                    storage: None,
                    nonce: None,
                    secret_key: None,
                },
            );
        }
    
        for (addr, account) in &genesis_config.el_premine_addrs {
            add_alloc_entry(&mut besu_genesis_config.alloc, addr, account);
        }
    
        for (addr, account) in &genesis_config.additional_preloaded_contracts {
            add_alloc_entry(&mut besu_genesis_config.alloc, addr, account);
        }
    
        if let Some(electra_fork_epoch) = genesis_config.electra_fork_epoch {
            besu_genesis_config.config.prague_time = Some(get_activation_epoch(
                genesis_config.genesis_timestamp,
                genesis_config.genesis_delay,
                &genesis_config.preset_base,
                genesis_config.slot_duration_in_seconds,
                electra_fork_epoch,
            ));
        }
    
        if let Some(eof_activation_epoch) = genesis_config.eof_activation_epoch {
            besu_genesis_config.config.prague_eoftime = Some(get_activation_epoch(
                genesis_config.genesis_timestamp,
                genesis_config.genesis_delay,
                &genesis_config.preset_base,
                genesis_config.slot_duration_in_seconds,
                eof_activation_epoch,
            ));
        }
    
        besu_genesis_config
    }
}

impl SerializableToFile for BesuGenesisConfig {}

fn add_alloc_entry(
    alloc: &mut std::collections::HashMap<String, Allocation>,
    addr: &String,
    account: &Allocation,
) {
    let alloc_entry = Allocation {
        balance: account.balance.replace("ETH", "000000000000000000"),
        code: account.code.clone(),
        storage: account.storage.clone(),
        nonce: account.nonce.clone(),
        secret_key: account.secret_key.clone(),
    };

    alloc.insert(addr.clone(), alloc_entry);
}

fn get_activation_epoch(
    genesis_timestamp: u64,
    genesis_delay: u64,
    preset_base: &str,
    slot_duration_in_seconds: u64,
    activation_epoch: u64,
) -> u64 {
    genesis_timestamp
        + genesis_delay
        + activation_epoch
            * (if preset_base == "mainnet" { 32 } else { 8 })
            * slot_duration_in_seconds
}
