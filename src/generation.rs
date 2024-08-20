use anyhow::Result;
use std::{collections::HashMap, fs};

use crate::el::serializabe_to_file::{Genesis, SerializableToFile};
use crate::el::{besu_generation::BesuGenesisConfig, genesis_config::GenesisConfig};

const METADATA_FOLDER: &str = "/tmp/metadata";
// const GETH_GENESIS: &str = "/tmp/metadata/genesis.json";
// const CHAINSPEC_GENESIS: &str = "/tmp/metadata/chainspec.json";
const BESU_GENESIS: &str = "/tmp/metadata/besu.json";

pub fn gen_el_config(context: HashMap<String, String>) -> Result<()> {
    // Create metadata directory
    fs::create_dir_all(METADATA_FOLDER)
        .expect(format!("Failed to create {}", METADATA_FOLDER).as_str());
    let genesis_config = GenesisConfig::try_from(context).unwrap();

    // GethGenesisConfig::create_genesis(&genesis_config).save_if_not_exists(GETH_GENESIS);

    // ChainspecGenesisConfig::create_genesis(&genesis_config).save_if_not_exists(CHAINSPEC_GENESIS);

    BesuGenesisConfig::create_genesis(&genesis_config).save_if_not_exists(BESU_GENESIS);

    Ok(())
}
