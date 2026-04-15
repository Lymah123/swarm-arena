use bevy::prelude::*;
use crate::components::*;

pub fn spawn_world(mut commands: Commands) {
    // spawn agent 0
    commands.spawn((
        AgentId(0),
        Position::new(0, 0),
        Score::default(),
        Action::Stay,
    ));
    crate::systems::on_chain::register_agent("swarm-agent-0", "~/.config/solana/id.json");

    // spawn agent 1
    commands.spawn((
        AgentId(1),
        Position::new(9, 9),
        Score::default(),
        Action::Stay,
    ));

    println!("Arena ready — 2 agents spawned on 10x10 grid");
}