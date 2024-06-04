use super::{add::run as add_metadata, remove::run as remove_metadata, MetadataArgs};

use anyhow::Result;
use clap::{Args, Subcommand};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

#[derive(Debug, Clone, Args)]
pub struct MetadataSubCommand {
    #[clap(subcommand)]
    pub action: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[clap(name = "add")]
    /// Add metadata for a mint account (member)
    Add(MetadataArgs),
    #[clap(name = "remove")]
    /// Remove metadata for a mint account (member)
    Remove(MetadataArgs),
}

pub async fn subcommand(
    async_client: RpcClient,
    keypair: Keypair,
    subcommand: MetadataSubCommand,
) -> Result<()> {
    match subcommand.action {
        Commands::Add(args) => {
            add_metadata(async_client, keypair, args).await?;
        }
        Commands::Remove(args) => {
            remove_metadata(async_client, keypair, args).await?;
        }
    }

    Ok(())
}