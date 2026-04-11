use crate::events::EpisodeEnd;
use bevy::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

const PROGRAM_ID: &str = "CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV";
const RPC_URL: &str = "http://127.0.0.1:8899";

pub fn commit_episode(mut events: EventReader<EpisodeEnd>) {
    for ep in events.read() {
        println!("Committing episode {} to Solana...", ep.episode_id);

        let client = RpcClient::new_with_commitment(
            RPC_URL.to_string(),
            CommitmentConfig::confirmed(),
        );

        let keypair = match read_keypair_file(
            &*shellexpand::tilde("~/.config/solana/id.json")
        ) {
            Ok(k) => k,
            Err(e) => { eprintln!("Failed to read keypair: {}", e); continue; }
        };

        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
        let episode_id_bytes = ep.episode_id.to_le_bytes();

        let (episode_pda, _) = Pubkey::find_program_address(
            &[b"episode", &episode_id_bytes],
            &program_id,
        );
        let (reputation_pda, _) = Pubkey::find_program_address(
            &[b"reputation", keypair.pubkey().as_ref()],
            &program_id,
        );

        let scores = [
            ep.scores.get(0).map(|(_, s)| *s as u64).unwrap_or(0),
            ep.scores.get(1).map(|(_, s)| *s as u64).unwrap_or(0),
        ];

        let discriminator: [u8; 8] = [117, 205, 78, 70, 141, 9, 232, 115];
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&ep.episode_id.to_le_bytes());
        data.extend_from_slice(&scores[0].to_le_bytes());
        data.extend_from_slice(&scores[1].to_le_bytes());
        data.extend_from_slice(&[0u8; 32]);

        println!("Data ({} bytes): {:?}", data.len(), data);

        let ix = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(episode_pda, false),
                AccountMeta::new(reputation_pda, false),
                AccountMeta::new(keypair.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
            data,
        };

        let recent_blockhash = match client.get_latest_blockhash() {
            Ok(bh) => bh,
            Err(e) => { eprintln!("Failed to get blockhash: {}", e); continue; }
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );

        match client.send_and_confirm_transaction(&tx) {
            Ok(sig) => println!("Episode {} committed! Sig: {}", ep.episode_id, sig),
            Err(e) => eprintln!("Transaction failed: {}", e),
        }
    }
}
