use super::create::{run as create_group_account, CreateArgs};

use anyhow::Result;
use clap::{Args, Subcommand};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

#[derive(Debug, Clone, Args)]
pub struct GroupSubCommand {
    #[clap(subcommand)]
    pub action: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[clap(name = "create")]
    Create(CreateArgs),
}

pub async fn subcommand(
    async_client: RpcClient,
    keypair: Keypair,
    subcommand: GroupSubCommand,
) -> Result<()> {
    match subcommand.action {
        Commands::Create(args) => {
            create_group_account(async_client, keypair, args).await?;
        }
    }

    Ok(())
}