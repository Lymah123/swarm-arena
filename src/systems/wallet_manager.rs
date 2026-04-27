use crate::components::*;
use crate::events::AgentRegistered;
use crate::resources::WalletRegistry;
use bevy::prelude::*;
use std::collections::HashSet;

/// System to track which wallets have been processed this frame
#[derive(Resource, Default)]
pub struct WalletConnections {
    pub pending_wallets: Vec<String>,
    pub processed_wallets: HashSet<String>,
}

impl WalletConnections {
    pub fn request_agent(&mut self, wallet: String) {
        if !self.processed_wallets.contains(&wallet) {
            self.pending_wallets.push(wallet);
        }
    }
}

/// Process pending wallet connections and spawn agents
pub fn process_wallet_registrations(
    mut wallet_connections: ResMut<WalletConnections>,
    mut commands: Commands,
    mut registry: ResMut<WalletRegistry>,
    mut ev_agent_registered: EventWriter<AgentRegistered>,
    grid_world: Res<crate::resources::GridWorld>,
) {
    let pending = std::mem::take(&mut wallet_connections.pending_wallets);

    for wallet in pending {
        // Check if wallet already has an agent
        if registry.get_agent_for_wallet(&wallet).is_some() {
            println!("Wallet {} already has an agent registered", wallet);
            wallet_connections.processed_wallets.insert(wallet);
            continue;
        }

        // Register new agent
        let agent_id = registry.register_agent(wallet.clone());

        // Find a random starting position
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(0..grid_world.width);
        let start_y = rng.gen_range(0..grid_world.height);

        // Spawn the agent entity
        commands.spawn((
            AgentId(agent_id),
            Wallet(wallet.clone()),
            Position::new(start_x, start_y),
            Score::default(),
            Action::Stay,
        ));

        // Emit event
        ev_agent_registered.send(AgentRegistered {
            agent_id,
            wallet: wallet.clone(),
        });

        // Register on-chain
        crate::systems::on_chain::register_agent(
            &format!("agent-{}", agent_id),
            "~/.config/solana/id.json",
        );

        println!("New agent #{} registered for wallet: {}", agent_id, wallet);
        wallet_connections.processed_wallets.insert(wallet);
    }
}

/// Provide agent info by wallet
pub fn get_agent_by_wallet(
    wallet: &str,
    registry: &WalletRegistry,
    query: &Query<(&AgentId, &Position, &Score, &Wallet)>,
) -> Option<(u8, i32, i32, f32)> {
    let agent_id = registry.get_agent_for_wallet(wallet)?;
    query
        .iter()
        .find(|(id, _, _, w)| id.0 == agent_id && w.0 == wallet)
        .map(|(id, pos, score, _)| (id.0, pos.x, pos.y, score.0))
}

/// Get all active agents with their stats
pub fn get_all_agents(
    registry: &WalletRegistry,
    query: &Query<(&AgentId, &Position, &Score, &Wallet)>,
) -> Vec<(u8, String, i32, i32, f32)> {
    query
        .iter()
        .map(|(id, pos, score, wallet)| (id.0, wallet.0.clone(), pos.x, pos.y, score.0))
        .collect()
}
