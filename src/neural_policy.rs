use bevy::prelude::*;
use crate::components::Action;
use crate::qtable::ACTIONS;
use sha2::{Sha256, Digest};
use std::collections::VecDeque;

const INPUT_SIZE: usize = 8;
const HIDDEN_SIZE: usize = 32;
const OUTPUT_SIZE: usize = 5;
const LR: f32 = 0.0005;
const GAMMA: f32 = 0.99;
const EPSILON_START: f32 = 1.0;
const EPSILON_END: f32 = 0.05;
const EPSILON_DECAY: f32 = 0.998;
const BUFFER_SIZE: usize = 10000;
const BATCH_SIZE: usize = 32;
const MIN_BUFFER: usize = 200;
const TARGET_UPDATE_FREQ: u32 = 100;

pub struct DQNTransition {
    pub state: Vec<f32>,
    pub action_idx: usize,
    pub reward: f32,
    pub next_state: Vec<f32>,
    pub done: bool,
}

pub struct ReplayBuffer {
    pub buffer: VecDeque<DQNTransition>,
    pub capacity: usize,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { buffer: VecDeque::with_capacity(capacity), capacity }
    }

    pub fn push(&mut self, t: DQNTransition) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(t);
    }

    pub fn len(&self) -> usize { self.buffer.len() }

    pub fn sample_indices(&self, batch_size: usize, rng: &mut u64) -> Vec<usize> {
        let n = self.buffer.len();
        let mut indices = Vec::with_capacity(batch_size);
        for _ in 0..batch_size {
            *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            indices.push((*rng >> 33) as usize % n);
        }
        indices
    }
}

#[derive(Debug, Clone)]
pub struct MLP {
    pub w1: Vec<Vec<f32>>,
    pub b1: Vec<f32>,
    pub w2: Vec<Vec<f32>>,
    pub b2: Vec<f32>,
    pub w3: Vec<Vec<f32>>,
    pub b3: Vec<f32>,
}

impl MLP {
    pub fn new(rng: &mut u64) -> Self {
        let mut rand_f32 = |rng: &mut u64| -> f32 {
            *rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let bits = 0x3F800000u32 | ((*rng >> 33) as u32 & 0x7FFFFF);
            f32::from_bits(bits) - 1.5
        };
        let s1 = (2.0f32 / INPUT_SIZE as f32).sqrt();
        let s2 = (2.0f32 / HIDDEN_SIZE as f32).sqrt();
        let w1 = (0..HIDDEN_SIZE).map(|_| (0..INPUT_SIZE).map(|_| rand_f32(rng) * s1).collect()).collect();
        let b1 = vec![0.0f32; HIDDEN_SIZE];
        let w2 = (0..HIDDEN_SIZE).map(|_| (0..HIDDEN_SIZE).map(|_| rand_f32(rng) * s2).collect()).collect();
        let b2 = vec![0.0f32; HIDDEN_SIZE];
        let w3 = (0..OUTPUT_SIZE).map(|_| (0..HIDDEN_SIZE).map(|_| rand_f32(rng) * s2).collect()).collect();
        let b3 = vec![0.0f32; OUTPUT_SIZE];
        Self { w1, b1, w2, b2, w3, b3 }
    }

    pub fn copy_from(&mut self, other: &MLP) {
        self.w1 = other.w1.clone();
        self.b1 = other.b1.clone();
        self.w2 = other.w2.clone();
        self.b2 = other.b2.clone();
        self.w3 = other.w3.clone();
        self.b3 = other.b3.clone();
    }

    fn relu(x: f32) -> f32 { x.max(0.0) }

    fn linear(w: &[Vec<f32>], b: &[f32], x: &[f32]) -> Vec<f32> {
        w.iter().zip(b.iter()).map(|(row, bi)| {
            row.iter().zip(x.iter()).map(|(wi, xi)| wi * xi).sum::<f32>() + bi
        }).collect()
    }

    pub fn q_values(&self, x: &[f32]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let h1_pre = Self::linear(&self.w1, &self.b1, x);
        let h1: Vec<f32> = h1_pre.iter().map(|v| Self::relu(*v)).collect();
        let h2_pre = Self::linear(&self.w2, &self.b2, &h1);
        let h2: Vec<f32> = h2_pre.iter().map(|v| Self::relu(*v)).collect();
        let q = Self::linear(&self.w3, &self.b3, &h2);
        (h1, h2, q)
    }

    pub fn has_bad_weights(&self) -> bool {
        let bad = |w: &f32| w.is_nan() || w.is_infinite();
        self.w1.iter().any(|r| r.iter().any(bad)) ||
        self.w2.iter().any(|r| r.iter().any(bad)) ||
        self.w3.iter().any(|r| r.iter().any(bad))
    }

    pub fn clip_weights(&mut self) {
        for row in &mut self.w1 { for w in row { *w = w.clamp(-5.0, 5.0); } }
        for row in &mut self.w2 { for w in row { *w = w.clamp(-5.0, 5.0); } }
        for row in &mut self.w3 { for w in row { *w = w.clamp(-5.0, 5.0); } }
        for b in &mut self.b1 { *b = b.clamp(-5.0, 5.0); }
        for b in &mut self.b2 { *b = b.clamp(-5.0, 5.0); }
        for b in &mut self.b3 { *b = b.clamp(-5.0, 5.0); }
    }

    pub fn weight_hash(&self) -> String {
        if self.has_bad_weights() { return "bad-weights".to_string(); }
        let mut hasher = Sha256::new();
        for row in &self.w1 { for w in row { hasher.update(w.to_le_bytes()); } }
        for row in &self.w2 { for w in row { hasher.update(w.to_le_bytes()); } }
        for row in &self.w3 { for w in row { hasher.update(w.to_le_bytes()); } }
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    pub fn update_dqn(&mut self, state: &[f32], action_idx: usize, td_error: f32) {
        if td_error.is_nan() || td_error.is_infinite() { return; }

        let (h1, h2, _) = self.q_values(state);
        let h1_pre: Vec<f32> = Self::linear(&self.w1, &self.b1, state);
        let h2_pre: Vec<f32> = Self::linear(&self.w2, &self.b2, &h1);

        // Only gradient for taken action, clipped
        let mut d_out = vec![0.0f32; OUTPUT_SIZE];
        d_out[action_idx] = (2.0 * td_error).clamp(-1.0, 1.0);

        // Update W3, b3
        for (i, row) in self.w3.iter_mut().enumerate() {
            for (j, w) in row.iter_mut().enumerate() { *w -= LR * d_out[i] * h2[j]; }
            self.b3[i] -= LR * d_out[i];
        }

        // Backprop to h2
        let d_h2: Vec<f32> = (0..HIDDEN_SIZE).map(|j| {
            self.w3.iter().zip(d_out.iter()).map(|(r, d)| r[j] * d).sum::<f32>()
        }).collect();
        let d_h2_pre: Vec<f32> = d_h2.iter().zip(h2_pre.iter())
            .map(|(d, hp)| if *hp > 0.0 { *d } else { 0.0 }).collect();

        if d_h2_pre.iter().any(|g| g.is_nan() || g.is_infinite()) { return; }

        // Update W2, b2
        for (i, row) in self.w2.iter_mut().enumerate() {
            for (j, w) in row.iter_mut().enumerate() { *w -= LR * d_h2_pre[i] * h1[j]; }
            self.b2[i] -= LR * d_h2_pre[i];
        }

        // Backprop to h1
        let d_h1: Vec<f32> = (0..HIDDEN_SIZE).map(|j| {
            self.w2.iter().zip(d_h2_pre.iter()).map(|(r, d)| r[j] * d).sum::<f32>()
        }).collect();
        let d_h1_pre: Vec<f32> = d_h1.iter().zip(h1_pre.iter())
            .map(|(d, hp)| if *hp > 0.0 { *d } else { 0.0 }).collect();

        if d_h1_pre.iter().any(|g| g.is_nan() || g.is_infinite()) { return; }

        // Update W1, b1
        for (i, row) in self.w1.iter_mut().enumerate() {
            for (j, w) in row.iter_mut().enumerate() { *w -= LR * d_h1_pre[i] * state[j]; }
            self.b1[i] -= LR * d_h1_pre[i];
        }

        self.clip_weights();
    }
}

#[derive(Resource)]
pub struct DQNAgent {
    pub online: MLP,
    pub target: MLP,
    pub buffer: ReplayBuffer,
    pub step: u32,
    pub episode_count: u32,
    pub epsilon: f32,
    pub episode_reward: f32,
    pub total_rewards: Vec<f32>,
    pub last_state: Option<Vec<f32>>,
    pub last_action_idx: Option<usize>,
    pub pending_reward: f32,
    pub rng: u64,
}

impl Default for DQNAgent {
    fn default() -> Self {
        let mut rng = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u64;
        let online = MLP::new(&mut rng);
        let target = online.clone();
        Self {
            online, target,
            buffer: ReplayBuffer::new(BUFFER_SIZE),
            step: 0, episode_count: 0,
            epsilon: EPSILON_START,
            episode_reward: 0.0,
            total_rewards: Vec::new(),
            last_state: None, last_action_idx: None,
            pending_reward: 0.0,
            rng,
        }
    }
}

impl DQNAgent {
    pub fn choose_action(&mut self, state: &[f32]) -> (usize, Action) {
        self.rng = self.rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let rand_val = (self.rng >> 33) as f32 / (u32::MAX as f32);
        if rand_val < self.epsilon {
            self.rng = self.rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let idx = (self.rng >> 29) as usize % ACTIONS.len();
            return (idx, ACTIONS[idx]);
        }
        let (_, _, q_vals) = self.online.q_values(state);
        let idx = q_vals.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i).unwrap_or(0);
        (idx, ACTIONS[idx])
    }

    pub fn add_reward(&mut self, r: f32) {
        self.pending_reward += r;
        self.episode_reward += r;
    }

    pub fn push_transition(&mut self, state: Vec<f32>, action_idx: usize, reward: f32, next_state: Vec<f32>, done: bool) {
        self.buffer.push(DQNTransition { state, action_idx, reward, next_state, done });
        self.train_step();
    }

    fn train_step(&mut self) {
        if self.buffer.len() < MIN_BUFFER { return; }
        let indices = self.buffer.sample_indices(BATCH_SIZE, &mut self.rng);
        for idx in indices {
            let t = &self.buffer.buffer[idx];
            let state = t.state.clone();
            let action_idx = t.action_idx;
            let reward = t.reward;
            let next_state = t.next_state.clone();
            let done = t.done;

            let (_, _, next_q) = self.target.q_values(&next_state);
            let max_next_q = if done { 0.0 } else {
                next_q.iter().cloned().fold(f32::NEG_INFINITY, f32::max).max(0.0)
            };
            let target_val = reward + GAMMA * max_next_q;
            let (_, _, current_q) = self.online.q_values(&state);
            let td_error = current_q[action_idx] - target_val;
            self.online.update_dqn(&state, action_idx, td_error);
        }
        self.step += 1;
        if self.step % TARGET_UPDATE_FREQ == 0 {
            let online = self.online.clone();
            self.target.copy_from(&online);
            println!("[DQN] Target updated at step {} — hash: {}", self.step, self.online.weight_hash());
        }
    }

    pub fn on_episode_end(&mut self, win_bonus: f32) {
        if let (Some(state), Some(action)) = (self.last_state.take(), self.last_action_idx.take()) {
            let reward = self.pending_reward + win_bonus;
            self.episode_reward += win_bonus;
            self.buffer.push(DQNTransition {
                state, action_idx: action, reward,
                next_state: vec![0.0; INPUT_SIZE], done: true,
            });
        }
        self.pending_reward = 0.0;
        self.epsilon = (self.epsilon * EPSILON_DECAY).max(EPSILON_END);
        self.total_rewards.push(self.episode_reward);
        self.episode_count += 1;
        self.episode_reward = 0.0;

        if self.episode_count % 10 == 0 {
            let recent: Vec<f32> = self.total_rewards.iter().rev().take(10).copied().collect();
            let avg = recent.iter().sum::<f32>() / recent.len() as f32;
            println!(
                "[DQN] ep {} — avg: {:.2} — eps: {:.3} — buf: {} — hash: {}",
                self.episode_count, avg, self.epsilon,
                self.buffer.len(), self.online.weight_hash()
            );
        }

        if self.online.has_bad_weights() {
            println!("[DQN] Bad weights — resetting");
            let mut rng = self.rng;
            self.online = MLP::new(&mut rng);
            self.target = self.online.clone();
            self.rng = rng;
        }
    }

    pub fn build_state(
        pos_x: i32, pos_y: i32,
        nearest_x: i32, nearest_y: i32,
        grid_width: i32, grid_height: i32,
        walls: &[(i32, i32)],
    ) -> Vec<f32> {
        let norm_x = pos_x as f32 / grid_width as f32;
        let norm_y = pos_y as f32 / grid_height as f32;
        let dx = (nearest_x - pos_x).signum() as f32;
        let dy = (nearest_y - pos_y).signum() as f32;
        let dist_x = (nearest_x - pos_x).abs() as f32 / grid_width as f32;
        let dist_y = (nearest_y - pos_y).abs() as f32 / grid_height as f32;
        let wall_up = (1..=grid_height).find(|d| {
            let ny = pos_y + d; ny >= grid_height || walls.contains(&(pos_x, ny))
        }).unwrap_or(grid_height) as f32 / grid_height as f32;
        let wall_down = (1..=grid_height).find(|d| {
            let ny = pos_y - d; ny < 0 || walls.contains(&(pos_x, ny))
        }).unwrap_or(grid_height) as f32 / grid_height as f32;
        vec![norm_x, norm_y, dx, dy, dist_x, dist_y, wall_up, wall_down]
    }
}
