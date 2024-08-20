use crate::{args::{ActionType, EthereumGenesisGeneratorBuilderArgs}, defaults_env::read_defaults, generation::gen_el_config};
use anyhow::{Context, Result};
use clap::Parser;

pub fn run_cli() -> Result<()> {
    let args = EthereumGenesisGeneratorBuilderArgs::try_parse()
        .context("Could not parse command line arguments")?;
    match args.action {
        ActionType::EL => {
            let context = read_defaults("defaults.env").context("Could not parse context file")?;
            gen_el_config(context)?;
            Ok(())
        }
        ActionType::All => {
            todo!();
           // Ok(())
        }
        ActionType::CL => {
            todo!();
            //Ok(())
        }
        ActionType::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}
