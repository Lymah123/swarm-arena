use bevy::prelude::*;
use crate::components::{Action, Position};
use crate::resources::GridWorld;

pub fn move_agents(
    mut query: Query<(&Action, &mut Position)>,
    grid: Res<GridWorld>,
) {
    for (action, mut pos) in &mut query {
        let (nx, ny) = match action {
            Action::Up    => (pos.x, pos.y + 1),
            Action::Down  => (pos.x, pos.y - 1),
            Action::Left  => (pos.x - 1, pos.y),
            Action::Right => (pos.x + 1, pos.y),
            Action::Stay  => (pos.x, pos.y),
        };

        if grid.in_bounds(nx, ny) {
            pos.x = nx;
            pos.y = ny;
        }
    }
}