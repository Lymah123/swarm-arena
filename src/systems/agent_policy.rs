use bevy::prelude::*;
use crate::components::{Action, AgentId, Position};
use crate::resources::GridWorld;
use crate::qtable::QTable;

pub fn assign_actions(
    mut agents: Query<(&AgentId, &Position, &mut Action)>,
    grid: Res<GridWorld>,
    mut qtable: ResMut<QTable>,
) {
    for (agent_id, pos, mut action) in &mut agents {
        if grid.resources.is_empty() {
            *action = Action::Stay;
            continue;
        }

        let nearest = nearest_resource(pos, &grid);

        match agent_id.0 {
            0 => {
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
            _ => {
                *action = heuristic_action(pos, nearest);
            }
        }
    }
}

pub fn on_resource_collected(
    agent_id: u8,
    pos_x: i32,
    pos_y: i32,
    nearest_x: i32,
    nearest_y: i32,
    qtable: &mut QTable,
) {
    if agent_id == 0 {
        let state = QTable::state_from_pos(pos_x, pos_y, nearest_x, nearest_y);
        if let (Some(prev_state), Some(prev_action)) =
            (qtable.last_state, qtable.last_action_idx)
        {
            qtable.update(prev_state, prev_action, 1.0, state);
        }
    }
}

fn nearest_resource(pos: &Position, grid: &GridWorld) -> (i32, i32) {
    grid.resources
        .iter()
        .min_by_key(|(rx, ry)| (rx - pos.x).abs() + (ry - pos.y).abs())
        .copied()
        .unwrap_or((pos.x, pos.y))
}

fn heuristic_action(pos: &Position, target: (i32, i32)) -> Action {
    let dx = target.0 - pos.x;
    let dy = target.1 - pos.y;
    if dx.abs() >= dy.abs() {
        if dx > 0 { Action::Right }
        else if dx < 0 { Action::Left }
        else { Action::Stay }
    } else {
        if dy > 0 { Action::Up }
        else if dy < 0 { Action::Down }
        else { Action::Stay }
    }
}
