use crate::components::*;
use crate::resources::WalletRegistry;
use crate::systems::wallet_manager::WalletConnections;
use bevy::prelude::*;

pub fn spawn_world(mut wallet_connections: ResMut<WalletConnections>) {
    // Initialize default demo agents
    let wallet0 = "9B5X4h3X7kX8vX9kX0X1X2X3X4X5X6X7X8X9XaX0".to_string();
    let wallet1 = "ETVgewbsk8EKDWFheVxbyWQyVgqsGukrntXjb2VL5Umq".to_string();

    wallet_connections.request_agent(wallet0);
    wallet_connections.request_agent(wallet1);

    println!("Arena initializing — waiting for wallet connections...");
}
