mod components;
mod events;
mod qtable;
mod neural_policy;
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
            if let Ok(map_path) = std::env::var("SWARM_MAP_FILE") {
                match resources::GridWorld::from_map_file(&map_path) {
                    Ok(world) => world,
                    Err(e) => {
                        eprintln!("Map file error: {}", e);
                        eprintln!("Falling back to random 10x10 grid");
                        resources::GridWorld::new(10, 10)
                    }
                }
            } else {
                let size = std::env::var("SWARM_GRID_SIZE")
                    .ok()
                    .and_then(|s| s.parse::<i32>().ok())
                    .unwrap_or(10)
                    .max(5)
                    .min(100);
                println!("Grid size: {}x{}", size, size);
                resources::GridWorld::new(size, size)
            }
        })
        .insert_resource(resources::WalletRegistry::default())
        .insert_resource(wallet_manager::WalletConnections::default())
        .insert_resource(qtable::QTable::default())
        .insert_resource(neural_policy::NeuralPolicy::default())
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
