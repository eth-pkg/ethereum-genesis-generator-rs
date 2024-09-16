use anyhow::{Context, Result};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::el::{besu_generation::BesuGenesisConfig, chainspec_generation::ChainspecGenesisConfig, genesis_config::{GenesisConfig, GenesisConfigBuilder}, geth_generation::GethGenesisConfig};
use crate::el::serializabe_to_file::{Genesis, SerializableToFile};


pub fn get_genesis_config(context: HashMap<String, String>) -> Result<GenesisConfig> {
    let genesis_config = GenesisConfigBuilder::new()
        .from_hashmap(context)?
        .with_default_premine()
        .build();
    return Ok(genesis_config);
}

pub fn gen_el_config(context: HashMap<String, String>, metadata_folder: PathBuf) -> Result<()> {
    fs::create_dir_all(&metadata_folder)
        .context("Failed to create metadata folder")?;

    let genesis_config = get_genesis_config(context)?;
    let geth_genesis_path = metadata_folder.join("genesis.json");
    GethGenesisConfig::create_genesis(&genesis_config).save_if_not_exists(geth_genesis_path);

    let chainspec_genesis_path = metadata_folder.join("chainspec.json");
    ChainspecGenesisConfig::create_genesis(&genesis_config).save_if_not_exists(chainspec_genesis_path);

    let besu_genesis_path = metadata_folder.join("besu.json");
    BesuGenesisConfig::create_genesis(&genesis_config).save_if_not_exists(besu_genesis_path);

    Ok(())
}

pub fn gen_cl_config(context: HashMap<String, String>, metadata_folder: PathBuf) -> Result<()> {
    todo!()
}

pub fn gen_shared_files(context: HashMap<String, String>, metadata_folder: PathBuf) -> Result<()> {
    todo!()
}