use bevy::prelude::*;
use serde::Serialize;

#[derive(Resource, Debug)]
pub struct EpisodeState {
    pub tick: u32,
    pub max_ticks: u32,
    pub episode_id: u64,
    pub done: bool,
}

impl Default for EpisodeState {
    fn default() -> Self {
        Self {
            tick: 0,
            max_ticks: 200,
            episode_id: 0,
            done: false,
        }
    }
}

impl EpisodeState {
    pub fn reset(&mut self) {
        self.tick = 0;
        self.done = false;
        self.episode_id += 1;
    }
}

#[derive(Resource, Debug)]
pub struct GridWorld {
    pub width: i32,
    pub height: i32,
    pub resources: Vec<(i32, i32)>,
}

impl GridWorld {
    pub fn new(width: i32, height: i32) -> Self {
        let resources = vec![
            (2, 2), (5, 5), (7, 3), (1, 8), (9, 1),
            (4, 6), (6, 4), (3, 7), (8, 8), (0, 5),
        ];
        Self { width, height, resources }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn collect_at(&mut self, x: i32, y: i32) -> bool {
        if let Some(idx) = self.resources.iter().position(|&r| r == (x, y)) {
            self.resources.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        *self = GridWorld::new(self.width, self.height);
    }
}

#[derive(Debug, Serialize)]
pub struct EpisodeResult {
    pub episode_id: u64,
    pub agent_scores: Vec<(u8, f32)>,
    pub ticks: u32,
}