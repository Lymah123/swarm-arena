use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct AgentRegistered {
    pub agent_id: u8,
    pub wallet: String,
}

#[derive(Event, Debug)]
pub struct EpisodeEnd {
    pub episode_id: u64,
    pub scores: Vec<(u8, f32)>,
    pub ticks: u32,
}