use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct EthereumGenesisGeneratorBuilderArgs {
    #[clap(subcommand)]
    pub action: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// creates  
    EL,
    /// creates consensus layer config files to run testnet
    CL,
    /// creates all configuration files required to run a testnet
    All,
    // pkg-builder version
    Version
}
