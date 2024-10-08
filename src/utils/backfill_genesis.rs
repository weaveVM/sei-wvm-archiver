use crate::utils::archive_block::archive;
use crate::utils::planetscale::ps_archive_block;
use crate::utils::schema::Network;
use crate::utils::env_var::get_env_var;
use anyhow::{Error, Ok};

pub async fn backfill_from_genesis() -> Result<(), Error> {
    let network = Network::config();
    let config_start_block = network.start_block;
    let backfill_block_start: String = get_env_var("backfill_start_block").unwrap_or(0.to_string());
    let backfill_blocks: Vec<u64> = (backfill_block_start.parse::<u64>().unwrap()..=config_start_block).collect();

    if config_start_block == 0 {
        return Ok(());
    }

    for &block_number in backfill_blocks.iter() {
        println!("\n{}", "#".repeat(100));
        println!(
            "\nARCHIVING **BACKFILL** BLOCK #{} of Network {} -- ChainId: {}\n",
            &block_number, network.name, network.network_chain_id
        );
        let archive_txid = archive(Some(block_number), true).await.unwrap();
        let _ = ps_archive_block(&block_number, &archive_txid).await;
        println!("\n{}", "#".repeat(100));
    }

    Ok(())
}
