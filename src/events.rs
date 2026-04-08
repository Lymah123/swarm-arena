use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct EpisodeEnd {
    pub episode_id: u64,
    pub scores: Vec<(u8, f32)>,
    pub ticks: u32,
}