use std::path::PathBuf;

use crate::{
    args::{ActionType, EthereumGenesisGeneratorBuilderArgs},
    defaults_env::read_defaults,
    generation::{gen_cl_config, gen_el_config, gen_shared_files},
};
use anyhow::{Context, Result};
use clap::Parser;

pub fn run_cli() -> Result<()> {
    let args = EthereumGenesisGeneratorBuilderArgs::try_parse()
        .context("Could not parse command line arguments")?;
    match args.action {
        ActionType::EL => {
            let context = read_defaults("defaults.env").context("Could not parse context file")?;
            gen_el_config(context, PathBuf::from("/tmp/metadata"))?;
            Ok(())
        }
        ActionType::All => {
            let context = read_defaults("defaults.env").context("Could not parse context file")?;
            gen_el_config(context.clone(), PathBuf::from("/tmp/metadata"))?;
            gen_cl_config(context.clone(), PathBuf::from("/tmp/metadata"))?;
            gen_shared_files(context, PathBuf::from("/tmp/metadata"))?;
            Ok(())
        }
        ActionType::CL => {
            let context = read_defaults("defaults.env").context("Could not parse context file")?;
            gen_cl_config(context, PathBuf::from("/tmp/metadata"))?;
            Ok(())
        }
        ActionType::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}
