use bevy::prelude::*;
use crate::components::Action;
use std::collections::HashMap;

pub type State = (i32, i32);

pub const ACTIONS: [Action; 5] = [
    Action::Up,
    Action::Down,
    Action::Left,
    Action::Right,
    Action::Stay,
];

const ALPHA: f32 = 0.3;
const GAMMA: f32 = 0.9;
const EPSILON: f32 = 0.15;

#[derive(Resource, Debug, Default)]
pub struct QTable {
    pub table: HashMap<(State, usize), f32>,
    pub episode_count: u32,
    pub last_state: Option<State>,
    pub last_action_idx: Option<usize>,
    pub episode_reward: f32,
    pub total_rewards: Vec<f32>,
}

impl QTable {
    pub fn get_q(&self, state: State, action_idx: usize) -> f32 {
        *self.table.get(&(state, action_idx)).unwrap_or(&0.0)
    }

    pub fn max_q(&self, state: State) -> f32 {
        (0..ACTIONS.len())
            .map(|i| self.get_q(state, i))
            .fold(f32::NEG_INFINITY, f32::max)
    }

    pub fn best_action_idx(&self, state: State) -> usize {
        (0..ACTIONS.len())
            .max_by(|&i, &j| {
                self.get_q(state, i)
                    .partial_cmp(&self.get_q(state, j))
                    .unwrap()
            })
            .unwrap_or(0)
    }

    pub fn choose_action(&self, state: State) -> (usize, Action) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();

        let explore = (nanos % 1000) as f32 / 1000.0 < EPSILON;

        let idx = if explore {
            nanos as usize % ACTIONS.len()
        } else {
            self.best_action_idx(state)
        };

        (idx, ACTIONS[idx])
    }

    pub fn update(&mut self, state: State, action_idx: usize, reward: f32, next_state: State) {
        let current_q = self.get_q(state, action_idx);
        let max_next_q = self.max_q(next_state);
        let new_q = current_q + ALPHA * (reward + GAMMA * max_next_q - current_q);
        self.table.insert((state, action_idx), new_q);
        self.episode_reward += reward;
    }

    pub fn end_episode(&mut self) {
        self.total_rewards.push(self.episode_reward);
        self.episode_count += 1;
        self.episode_reward = 0.0;
        self.last_state = None;
        self.last_action_idx = None;
    }

    pub fn state_from_pos(pos_x: i32, pos_y: i32, target_x: i32, target_y: i32) -> State {
        let dx = (target_x - pos_x).signum();
        let dy = (target_y - pos_y).signum();
        (dx, dy)
    }
}
