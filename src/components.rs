use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Wallet(pub String);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct AgentId(pub u8);

#[derive(Component, Debug, Default)]
pub struct Score(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Stay,
}

impl Action {
    pub fn random() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        match t % 5 {
            0 => Action::Up,
            1 => Action::Down,
            2 => Action::Left,
            3 => Action::Right,
            _ => Action::Stay,
        }
    }
}

#[derive(Component, Debug)]
pub struct Resource;