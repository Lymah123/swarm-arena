use sha2::{Digest, Sha256};
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
const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    let keypair_path = std::env::var("SWARM_KEYPAIR")
        .unwrap_or_else(|_| "~/.config/solana/id.json".to_string());
    let agent_name = std::env::var("SWARM_AGENT_NAME")
        .unwrap_or_else(|_| "my-agent".to_string());
    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());
    let keypair = match read_keypair_file(&*shellexpand::tilde(&keypair_path)) {
        Ok(k) => k,
        Err(e) => { eprintln!("Failed to read keypair: {}", e); std::process::exit(1); }
    };
    println!("Registering agent {} for wallet {}...", agent_name, keypair.pubkey());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let (agent_pda, _) = Pubkey::find_program_address(&[b"agent", keypair.pubkey().as_ref()], &program_id);
    let mut hasher = Sha256::new();
    hasher.update(b"global:create_agent");
    let hash = hasher.finalize();
    let discriminator: [u8; 8] = hash[..8].try_into().unwrap();
    let name_bytes = agent_name.as_bytes();
    let mut data = discriminator.to_vec();
    data.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
    data.extend_from_slice(name_bytes);
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(agent_pda, false),
            AccountMeta::new(keypair.pubkey(), true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    };
    let recent_blockhash = match client.get_latest_blockhash() {
        Ok(bh) => bh,
        Err(e) => { eprintln!("Failed to get blockhash: {}", e); std::process::exit(1); }
    };
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&keypair.pubkey()), &[&keypair], recent_blockhash);
    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => {
            println!("Agent {} registered!", agent_name);
            println!("  Wallet:    {}", keypair.pubkey());
            println!("  Agent PDA: {}", agent_pda);
            println!("  Explorer:  https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            println!("  Next: SWARM_KEYPAIR={} cargo run --bin swarm-arena", keypair_path);
        }
        Err(e) => {
            eprintln!("Registration failed: {}", e);
            eprintln!("Get devnet SOL: solana airdrop 2 --url devnet");
            std::process::exit(1);
        }
    }
}
