use bevy::prelude::*;
use crate::components::{Action, AgentId, Position};
use crate::resources::GridWorld;
use crate::qtable::QTable;
use crate::neural_policy::DQNAgent;

pub fn assign_actions(
    mut agents: Query<(&AgentId, &Position, &mut Action)>,
    grid: Res<GridWorld>,
    mut qtable: ResMut<QTable>,
    mut dqn: ResMut<DQNAgent>,
) {
    for (agent_id, pos, mut action) in &mut agents {
        if grid.resources.is_empty() {
            *action = Action::Stay;
            continue;
        }
        let nearest = nearest_resource(pos, &grid);
        match agent_id.0 {
            0 => {
                let state = DQNAgent::build_state(
                    pos.x, pos.y, nearest.0, nearest.1,
                    grid.width, grid.height, &grid.walls,
                );
                // Push previous transition now that we have next_state
                if let (Some(prev_state), Some(prev_action)) =
                    (dqn.last_state.clone(), dqn.last_action_idx)
                {
                    let reward = dqn.pending_reward;
                    dqn.push_transition(prev_state, prev_action, reward, state.clone(), false);
                    dqn.pending_reward = 0.0;
                }
                let (action_idx, chosen) = dqn.choose_action(&state);
                dqn.last_state = Some(state);
                dqn.last_action_idx = Some(action_idx);
                *action = chosen;
            }
            1 => {
                let state = QTable::state_from_pos(pos.x, pos.y, nearest.0, nearest.1);
                if let (Some(prev_state), Some(prev_action)) =
                    (qtable.last_state, qtable.last_action_idx)
                {
                    qtable.update(prev_state, prev_action, 0.0, state);
                }
                let (action_idx, chosen) = qtable.choose_action(state);
                qtable.last_state = Some(state);
                qtable.last_action_idx = Some(action_idx);
                *action = chosen;
            }
            _ => { *action = heuristic_action(pos, nearest); }
        }
    }
}

fn nearest_resource(pos: &Position, grid: &GridWorld) -> (i32, i32) {
    grid.resources.iter()
        .min_by_key(|(rx, ry)| (rx - pos.x).abs() + (ry - pos.y).abs())
        .copied().unwrap_or((pos.x, pos.y))
}

fn heuristic_action(pos: &Position, target: (i32, i32)) -> Action {
    let dx = target.0 - pos.x;
    let dy = target.1 - pos.y;
    if dx.abs() >= dy.abs() {
        if dx > 0 { Action::Right } else if dx < 0 { Action::Left } else { Action::Stay }
    } else {
        if dy > 0 { Action::Up } else if dy < 0 { Action::Down } else { Action::Stay }
    }
}
