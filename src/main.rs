mod components;
mod resources;
mod events;
mod systems;

use bevy::prelude::*;
use systems::{setup, agent_policy, movement, rewards, episode, on_chain};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<events::EpisodeEnd>()
        .insert_resource(resources::EpisodeState::default())
        .insert_resource(resources::GridWorld::new(10, 10))
        .add_systems(Startup, setup::spawn_world)
        .add_systems(Update, (
            agent_policy::assign_actions,
            movement::move_agents,
            rewards::collect_resources,
            episode::tick_episode,
            episode::tick_episode,
            on_chain::commit_episode,
        ).chain())
        .run();
}
