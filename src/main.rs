mod components;
mod events;
mod qtable;
mod resources;
mod systems;

use bevy::prelude::*;
use systems::{agent_policy, episode, movement, on_chain, rewards, setup, wallet_manager};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<events::EpisodeEnd>()
        .add_event::<events::AgentRegistered>()
        .insert_resource(resources::EpisodeState::default())
        .insert_resource({
            let size = std::env::var("SWARM_GRID_SIZE")
                .ok()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(10)
                .max(5)
                .min(100);
            println!("Grid size: {}x{}", size, size);
            resources::GridWorld::new(size, size)
        })
        .insert_resource(resources::WalletRegistry::default())
        .insert_resource(wallet_manager::WalletConnections::default())
        .insert_resource(qtable::QTable::default())
        .add_systems(Startup, setup::spawn_world)
        .add_systems(
            Update,
            (
                wallet_manager::process_wallet_registrations,
                agent_policy::assign_actions,
                movement::move_agents,
                rewards::collect_resources,
                episode::tick_episode,
                on_chain::commit_episode,
            )
                .chain(),
        )
        .run();
}
