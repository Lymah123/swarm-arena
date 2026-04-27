use bevy::prelude::*;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Resource, Debug)]
pub struct WalletRegistry {
    /// Maps wallet address (Solana pubkey string) to agent ID
    pub wallet_to_agent: HashMap<String, u8>,
    /// Maps agent ID to wallet address
    pub agent_to_wallet: HashMap<u8, String>,
    /// Next available agent ID
    pub next_agent_id: u8,
}

impl Default for WalletRegistry {
    fn default() -> Self {
        Self {
            wallet_to_agent: HashMap::new(),
            agent_to_wallet: HashMap::new(),
            next_agent_id: 0,
        }
    }
}

impl WalletRegistry {
    pub fn register_agent(&mut self, wallet: String) -> u8 {
        if let Some(agent_id) = self.wallet_to_agent.get(&wallet) {
            *agent_id
        } else {
            let agent_id = self.next_agent_id;
            self.wallet_to_agent.insert(wallet.clone(), agent_id);
            self.agent_to_wallet.insert(agent_id, wallet);
            self.next_agent_id += 1;
            agent_id
        }
    }

    pub fn get_agent_for_wallet(&self, wallet: &str) -> Option<u8> {
        self.wallet_to_agent.get(wallet).copied()
    }

    pub fn get_wallet_for_agent(&self, agent_id: u8) -> Option<&str> {
        self.agent_to_wallet.get(&agent_id).map(|s| s.as_str())
    }

    pub fn agent_count(&self) -> u8 {
        self.next_agent_id
    }
}

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
            episode_id: 10000,
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
        let mut rng = rand::thread_rng();
        let num_resources = 10;
        let mut resources = Vec::new();

        // Generate random unique resource positions
        while resources.len() < num_resources {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            let pos = (x, y);

            if !resources.contains(&pos) {
                resources.push(pos);
            }
        }

        Self {
            width,
            height,
            resources,
        }
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
