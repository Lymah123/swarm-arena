use bevy::prelude::*;
use crate::components::{AgentId, Position, Score};
use crate::resources::GridWorld;
use crate::qtable::QTable;
use crate::neural_policy::DQNAgent;

pub fn collect_resources(
    mut agents: Query<(&AgentId, &Position, &mut Score)>,
    mut grid: ResMut<GridWorld>,
    mut qtable: ResMut<QTable>,
    mut dqn: ResMut<DQNAgent>,
) {
    for (id, pos, mut score) in &mut agents {
        if grid.collect_at(pos.x, pos.y) {
            score.0 += 1.0;
            println!("Agent {} collected resource at ({},{}) — score: {}", id.0, pos.x, pos.y, score.0);
            match id.0 {
                0 => { dqn.add_reward(1.0); }
                1 => {
                    let nearest = grid.resources.iter()
                        .min_by_key(|(rx, ry)| (rx - pos.x).abs() + (ry - pos.y).abs())
                        .copied().unwrap_or((pos.x, pos.y));
                    let state = QTable::state_from_pos(pos.x, pos.y, nearest.0, nearest.1);
                    if let (Some(ps), Some(pa)) = (qtable.last_state, qtable.last_action_idx) {
                        qtable.update(ps, pa, 1.0, state);
                    }
                }
                _ => {}
            }
        }
    }
}
