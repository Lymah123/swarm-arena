use bevy::prelude::*;
use crate::components::{Action, AgentId, Position};
use crate::resources::GridWorld;

pub fn assign_actions(
    mut query: Query<(&AgentId, &Position, &mut Action)>,
    grid: Res<GridWorld>,
) {
    for (_, pos, mut action) in &mut query {
        *action = best_action(pos, &grid);
    }
}

fn best_action(pos: &Position, grid: &GridWorld) -> Action {
    if grid.resources.is_empty() {
        return Action::Stay;
    }

    // find nearest resource
    let target = grid.resources
        .iter()
        .min_by_key(|(rx, ry)| {
            let dx = (rx - pos.x).abs();
            let dy = (ry - pos.y).abs();
            dx + dy
        })
        .copied();

    let (tx, ty) = match target {
        Some(t) => t,
        None => return Action::Stay,
    };

    let dx = tx - pos.x;
    let dy = ty - pos.y;

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
