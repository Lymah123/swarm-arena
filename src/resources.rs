use bevy::prelude::*;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Resource, Debug)]
pub struct WalletRegistry {
    pub wallet_to_agent: HashMap<String, u8>,
    pub agent_to_wallet: HashMap<u8, String>,
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

#[derive(Resource, Debug, Clone)]
pub struct GridWorld {
    pub width: i32,
    pub height: i32,
    pub resources: Vec<(i32, i32)>,
    pub walls: Vec<(i32, i32)>,
}

impl GridWorld {
    pub fn new(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let num_resources = ((width * height) / 10).max(10).min(50) as usize;
        let mut resources = Vec::new();
        while resources.len() < num_resources {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            let pos = (x, y);
            if !resources.contains(&pos) {
                resources.push(pos);
            }
        }
        Self { width, height, resources, walls: Vec::new() }
    }

    pub fn from_map_file(map_path: &str) -> Result<Self, String> {
        let raw = std::fs::read_to_string(map_path)
            .map_err(|e| format!("Could not read map file {}: {}", map_path, e))?;

        let lines: Vec<&str> = raw
            .lines()
            .filter(|l| !l.starts_with("//") && !l.trim().is_empty())
            .collect();

        if lines.is_empty() {
            return Err("Map file is empty".to_string());
        }

        let width = lines[0].len() as i32;
        let height = lines.len() as i32;

        for (i, line) in lines.iter().enumerate() {
            if line.len() as i32 != width {
                return Err(format!(
                    "Line {} has width {} but expected {}",
                    i + 1, line.len(), width
                ));
            }
        }

        let mut resources = Vec::new();
        let mut walls = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    'R' => resources.push((x as i32, y as i32)),
                    '.' => {}
                    '#' => walls.push((x as i32, y as i32)),
                    other => return Err(format!(
                        "Unknown character '{}' at ({}, {})", other, x, y
                    )),
                }
            }
        }

        if resources.is_empty() {
            return Err("Map has no resource positions (R)".to_string());
        }

        println!("Loaded map: {}x{} with {} resources, {} walls", width, height, resources.len(), walls.len());
        Ok(Self { width, height, resources, walls })
    }

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        self.walls.contains(&(x, y))
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
        let walls = self.walls.clone();
        *self = GridWorld::new(self.width, self.height);
        self.walls = walls;
    }
}

#[derive(Debug, Serialize)]
pub struct EpisodeResult {
    pub episode_id: u64,
    pub agent_scores: Vec<(u8, f32)>,
    pub ticks: u32,
}
