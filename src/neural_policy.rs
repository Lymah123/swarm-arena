use bevy::prelude::*;
use crate::components::Action;
use crate::qtable::ACTIONS;
use sha2::{Sha256, Digest};

const INPUT_SIZE: usize = 8;
const HIDDEN_SIZE: usize = 32;
const OUTPUT_SIZE: usize = 5;
const LR: f32 = 0.001;
const GAMMA: f32 = 0.99;
const EPSILON: f32 = 0.10;

// ---------- MLP ----------

#[derive(Debug)]
pub struct MLP {
    pub w1: Vec<Vec<f32>>,
    pub b1: Vec<f32>,
    pub w2: Vec<Vec<f32>>,
    pub b2: Vec<f32>,
    pub w3: Vec<Vec<f32>>,
    pub b3: Vec<f32>,
}

impl MLP {
    pub fn new() -> Self {
        let mut s = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u64;

        let mut rand_f32 = move || -> f32 {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let bits = 0x3F800000u32 | ((s >> 33) as u32 & 0x7FFFFF);
            f32::from_bits(bits) - 1.5
        };

        let s1 = (2.0f32 / INPUT_SIZE as f32).sqrt();
        let s2 = (2.0f32 / HIDDEN_SIZE as f32).sqrt();

        let w1 = (0..HIDDEN_SIZE).map(|_| (0..INPUT_SIZE).map(|_| rand_f32() * s1).collect()).collect();
        let b1 = vec![0.0f32; HIDDEN_SIZE];
        let w2 = (0..HIDDEN_SIZE).map(|_| (0..HIDDEN_SIZE).map(|_| rand_f32() * s2).collect()).collect();
        let b2 = vec![0.0f32; HIDDEN_SIZE];
        let w3 = (0..OUTPUT_SIZE).map(|_| (0..HIDDEN_SIZE).map(|_| rand_f32() * s2).collect()).collect();
        let b3 = vec![0.0f32; OUTPUT_SIZE];

        Self { w1, b1, w2, b2, w3, b3 }
    }

    fn relu(x: f32) -> f32 { x.max(0.0) }

    fn softmax(v: &[f32]) -> Vec<f32> {
        let m = v.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let e: Vec<f32> = v.iter().map(|x| (x - m).exp()).collect();
        let s: f32 = e.iter().sum();
        e.iter().map(|x| x / s).collect()
    }

    fn linear(w: &[Vec<f32>], b: &[f32], x: &[f32]) -> Vec<f32> {
        w.iter().zip(b.iter()).map(|(row, bi)| {
            row.iter().zip(x.iter()).map(|(wi, xi)| wi * xi).sum::<f32>() + bi
        }).collect()
    }

    pub fn forward(&self, x: &[f32]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let h1_pre = Self::linear(&self.w1, &self.b1, x);
        let h1: Vec<f32> = h1_pre.iter().map(|v| Self::relu(*v)).collect();
        let h2_pre = Self::linear(&self.w2, &self.b2, &h1);
        let h2: Vec<f32> = h2_pre.iter().map(|v| Self::relu(*v)).collect();
        let logits = Self::linear(&self.w3, &self.b3, &h2);
        let probs = Self::softmax(&logits);
        (h1, h2, probs)
    }

    pub fn predict(&self, x: &[f32]) -> Vec<f32> {
        self.forward(x).2
    }

    pub fn has_nan(&self) -> bool {
        let bad = |w: &f32| w.is_nan() || w.is_infinite();
        self.w1.iter().any(|r| r.iter().any(bad)) ||
        self.w2.iter().any(|r| r.iter().any(bad)) ||
        self.w3.iter().any(|r| r.iter().any(bad))
    }

    fn clip_weights(&mut self) {
        let clip = |w: &mut f32| { *w = w.clamp(-5.0, 5.0); };
        for row in &mut self.w1 { for w in row { clip(w); } }
        for row in &mut self.w2 { for w in row { clip(w); } }
        for row in &mut self.w3 { for w in row { clip(w); } }
        for b in &mut self.b1 { clip(b); }
        for b in &mut self.b2 { clip(b); }
        for b in &mut self.b3 { clip(b); }
    }

    pub fn weight_hash(&self) -> String {
        if self.has_nan() { return "nan-weights-reset".to_string(); }
        let mut hasher = Sha256::new();
        for row in &self.w1 { for w in row { hasher.update(w.to_le_bytes()); } }
        for row in &self.w2 { for w in row { hasher.update(w.to_le_bytes()); } }
        for row in &self.w3 { for w in row { hasher.update(w.to_le_bytes()); } }
        format!("{:x}", hasher.finalize())[..16].to_string()
    }
}

// ---------- Transition buffer ----------

#[derive(Debug)]
pub struct Transition {
    pub state: Vec<f32>,
    pub action_idx: usize,
    pub reward: f32,
}

// ---------- NeuralPolicy resource ----------

#[derive(Resource, Debug)]
pub struct NeuralPolicy {
    pub model: MLP,
    pub trajectory: Vec<Transition>,
    pub episode_count: u32,
    pub episode_reward: f32,
    pub total_rewards: Vec<f32>,
    pub last_state: Option<Vec<f32>>,
    pub last_action_idx: Option<usize>,
}

impl Default for NeuralPolicy {
    fn default() -> Self {
        Self {
            model: MLP::new(),
            trajectory: Vec::new(),
            episode_count: 0,
            episode_reward: 0.0,
            total_rewards: Vec::new(),
            last_state: None,
            last_action_idx: None,
        }
    }
}

impl NeuralPolicy {
    pub fn choose_action(&self, state: &[f32]) -> (usize, Action) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();

        if (nanos % 1000) as f32 / 1000.0 < EPSILON {
            let idx = nanos as usize % ACTIONS.len();
            return (idx, ACTIONS[idx]);
        }

        let probs = self.model.predict(state);
        let threshold = (nanos % 10000) as f32 / 10000.0;
        let mut cumulative = 0.0f32;
        let mut chosen = ACTIONS.len() - 1;
        for (i, p) in probs.iter().enumerate() {
            cumulative += p;
            if threshold <= cumulative {
                chosen = i;
                break;
            }
        }
        (chosen, ACTIONS[chosen])
    }

    pub fn record_step(&mut self, state: Vec<f32>, action_idx: usize, reward: f32) {
        self.trajectory.push(Transition { state, action_idx, reward });
        self.episode_reward += reward;
    }

    pub fn update_on_episode_end(&mut self) {
        if self.trajectory.is_empty() { return; }

        // Detect and recover from NaN weights
        if self.model.has_nan() {
            println!("[NEURAL] NaN weights detected — resetting model");
            self.model = MLP::new();
            self.trajectory.clear();
            return;
        }

        // Compute discounted returns
        let n = self.trajectory.len();
        let mut returns = vec![0.0f32; n];
        let mut g = 0.0f32;
        for i in (0..n).rev() {
            g = self.trajectory[i].reward + GAMMA * g;
            returns[i] = g;
        }

        // Normalize
        let mean = returns.iter().sum::<f32>() / n as f32;
        let std = (returns.iter().map(|r| (r - mean).powi(2)).sum::<f32>() / n as f32).sqrt() + 1e-8;
        let normalized: Vec<f32> = returns.iter().map(|r| (r - mean) / std).collect();

        // REINFORCE gradient update
        for (t, tr) in self.trajectory.iter().enumerate() {
            let (h1, h2, probs) = self.model.forward(&tr.state);
            let advantage = normalized[t];

            // d log pi / d logits = 1[a] - pi
            let mut d_logits: Vec<f32> = probs.iter().map(|p| -advantage * LR * (-p)).collect();
            d_logits[tr.action_idx] = -advantage * LR * (1.0 - probs[tr.action_idx]);

            // Entropy bonus — encourage exploration
            let entropy_bonus = 0.01f32;
            let mut d_logits_with_entropy: Vec<f32> = d_logits.iter().zip(probs.iter())
                .map(|(d, p)| d - entropy_bonus * (1.0 + p.ln().max(-10.0)))
                .collect();

            // Gradient clipping — max norm 1.0
            let grad_norm = d_logits_with_entropy.iter().map(|g| g * g).sum::<f32>().sqrt();
            if grad_norm > 1.0 {
                let scale = 1.0 / grad_norm;
                for g in d_logits_with_entropy.iter_mut() { *g *= scale; }
            }
            let d_logits = d_logits_with_entropy;

            // NaN guard
            if d_logits.iter().any(|g| g.is_nan() || g.is_infinite()) {
                continue;
            }

            // Update w3, b3
            for (i, row) in self.model.w3.iter_mut().enumerate() {
                for (j, w) in row.iter_mut().enumerate() { *w -= d_logits[i] * h2[j]; }
                self.model.b3[i] -= d_logits[i];
            }

            // Backprop to h2
            let d_h2: Vec<f32> = (0..HIDDEN_SIZE).map(|j| {
                self.model.w3.iter().zip(d_logits.iter()).map(|(r, d)| r[j] * d).sum::<f32>()
            }).collect();
            let d_h2_pre: Vec<f32> = d_h2.iter().zip(h2.iter())
                .map(|(d, h)| if *h > 0.0 { *d } else { 0.0 }).collect();

            // Guard hidden gradients
            if d_h2_pre.iter().any(|g| g.is_nan() || g.is_infinite()) { continue; }

            // Update w2, b2
            for (i, row) in self.model.w2.iter_mut().enumerate() {
                for (j, w) in row.iter_mut().enumerate() { *w -= d_h2_pre[i] * h1[j]; }
                self.model.b2[i] -= d_h2_pre[i];
            }

            // Backprop to h1
            let d_h1: Vec<f32> = (0..HIDDEN_SIZE).map(|j| {
                self.model.w2.iter().zip(d_h2_pre.iter()).map(|(r, d)| r[j] * d).sum::<f32>()
            }).collect();
            let d_h1_pre: Vec<f32> = d_h1.iter().zip(h1.iter())
                .map(|(d, h)| if *h > 0.0 { *d } else { 0.0 }).collect();

            if d_h1_pre.iter().any(|g| g.is_nan() || g.is_infinite()) { continue; }

            // Update w1, b1
            for (i, row) in self.model.w1.iter_mut().enumerate() {
                for (j, w) in row.iter_mut().enumerate() { *w -= d_h1_pre[i] * tr.state[j]; }
                self.model.b1[i] -= d_h1_pre[i];
            }
        }

        // Clip weights to prevent explosion
        self.model.clip_weights();

        self.total_rewards.push(self.episode_reward);
        self.episode_count += 1;
        self.episode_reward = 0.0;
        self.trajectory.clear();
        self.last_state = None;
        self.last_action_idx = None;

        if self.episode_count % 10 == 0 {
            let recent: Vec<f32> = self.total_rewards.iter().rev().take(10).copied().collect();
            let avg = recent.iter().sum::<f32>() / recent.len() as f32;
            println!(
                "[NEURAL] ep {} — avg reward last 10: {:.2} — model hash: {}",
                self.episode_count, avg, self.model.weight_hash()
            );
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
