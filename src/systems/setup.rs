use crate::systems::wallet_manager::WalletConnections;
use bevy::prelude::*;
use solana_sdk::signature::{read_keypair_file, Signer};
use shellexpand;

pub fn spawn_world(mut wallet_connections: ResMut<WalletConnections>) {
    let keypair_path = std::env::var("SWARM_KEYPAIR")
        .unwrap_or_else(|_| "~/.config/solana/id.json".to_string());

    let wallet = match read_keypair_file(&*shellexpand::tilde(&keypair_path)) {
        Ok(kp) => {
            println!("Loaded keypair: {}", kp.pubkey());
            kp.pubkey().to_string()
        }
        Err(e) => {
            eprintln!("Could not read keypair from '{}': {}", keypair_path, e);
            eprintln!("Set SWARM_KEYPAIR env var to your keypair path.");
            eprintln!("Falling back to default wallet.");
            "ETVgewbsk8EKDWFheVxbyWQyVgqsGukrntXjb2VL5Umq".to_string()
        }
    };

    wallet_connections.request_agent(wallet);
    println!("Arena initializing — waiting for wallet connections...");
}
