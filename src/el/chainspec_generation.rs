use ethers::signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use super::{
    config::{Allocation, Config},
    genesis_config::GenesisConfig,
    serializabe_to_file::{Genesis, SerializableToFile},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainspecGenesisConfig {
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

impl ChainspecGenesisConfig {
    fn for_mainnet() -> Self {
        let mainnet_config_path = "../data/mainnet/chainspec_genesis.json";
        serde_json::from_reader(fs::File::open(mainnet_config_path).unwrap()).unwrap()
    }
    fn for_holesky() -> Self {
        let holesky_config_path = "../data/holesky/chainspec_genesis.json";
        serde_json::from_reader(fs::File::open(holesky_config_path).unwrap()).unwrap()
    }
    fn for_sepolia() -> Self {
        let sepolia_config_path = "../data/sepolia/chainspec_genesis.json";
        serde_json::from_reader(fs::File::open(sepolia_config_path).unwrap()).unwrap()
    }
    fn for_devnet(genesis_config: &GenesisConfig) -> Self {
        let devnet_config_path = "../data/devnet/chainspec_genesis.json";
        let mut devnet_config: ChainspecGenesisConfig =
            serde_json::from_reader(fs::File::open(devnet_config_path).unwrap()).unwrap();

        // todo replace values
        devnet_config.config.chain_id = genesis_config.chain_id;
        devnet_config.config.deposit_contract_address = genesis_config.deposit_contract_address.clone();
        devnet_config.timestamp = genesis_config.genesis_timestamp.to_string();
        // TODO 
        //devnet_config.gas_limit

        devnet_config
    }
}

struct ChainspecGenesisConfigBuilder {
    config: ChainspecGenesisConfig,
}

impl ChainspecGenesisConfigBuilder {
    fn new(genesis_config: &GenesisConfig) -> Self {
        let config = match genesis_config.chain_id {
            1 => ChainspecGenesisConfig::for_mainnet(),
            11155111 => ChainspecGenesisConfig::for_sepolia(),
            17000 => ChainspecGenesisConfig::for_holesky(),
            _ => ChainspecGenesisConfig::for_devnet(&genesis_config),
        };
        ChainspecGenesisConfigBuilder { config }
    }

    fn with_premine(mut self, genesis_config: &GenesisConfig) -> Self {
        for (_, value) in &genesis_config.el_premine {
            let wallet: LocalWallet = MnemonicBuilder::<English>::default()
                .phrase(genesis_config.mnemonic.as_str())
                .build()
                .unwrap();

            let weival = value.replace("ETH", "000000000000000000");
            self.config.alloc.insert(
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
        self
    }

    fn with_premine_addrs(mut self, genesis_config: &GenesisConfig) -> Self {
        for (addr, account) in &genesis_config.el_premine_addrs {
            add_alloc_entry(&mut self.config.alloc, addr, account);
        }
        self
    }

    fn with_additional_preloaded_contracts(mut self, genesis_config: &GenesisConfig) -> Self {
        for (addr, account) in &genesis_config.additional_preloaded_contracts {
            add_alloc_entry(&mut self.config.alloc, addr, account);
        }
        self
    }

    fn with_electra_fork_epoch(mut self, genesis_config: &GenesisConfig) -> Self {
        if let Some(electra_fork_epoch) = genesis_config.electra_fork_epoch {
            self.config.config.prague_time = Some(get_activation_epoch(
                genesis_config.genesis_timestamp,
                genesis_config.genesis_delay,
                &genesis_config.preset_base,
                genesis_config.slot_duration_in_seconds,
                electra_fork_epoch,
            ));
        }
        self
    }

    fn with_eof_activation_epoch(mut self, genesis_config: &GenesisConfig) -> Self {
        if let Some(eof_activation_epoch) = genesis_config.eof_activation_epoch {
            self.config.config.prague_eoftime = Some(get_activation_epoch(
                genesis_config.genesis_timestamp,
                genesis_config.genesis_delay,
                &genesis_config.preset_base,
                genesis_config.slot_duration_in_seconds,
                eof_activation_epoch,
            ));
        }
        self
    }

    fn build(self) -> ChainspecGenesisConfig {
        self.config
    }
}

impl Genesis for ChainspecGenesisConfig {
    fn create_genesis(genesis_config: &GenesisConfig) -> ChainspecGenesisConfig {
        ChainspecGenesisConfigBuilder::new(genesis_config)
            .with_premine(genesis_config)
            .with_premine_addrs(genesis_config)
            .with_additional_preloaded_contracts(genesis_config)
            .with_electra_fork_epoch(genesis_config)
            .with_eof_activation_epoch(genesis_config)
            .build()
    }
}

impl SerializableToFile for ChainspecGenesisConfig {}

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
