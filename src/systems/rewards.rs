use bevy::prelude::*;
use crate::components::{AgentId, Position, Score};
use crate::resources::GridWorld;

pub fn collect_resources(
    mut agents: Query<(&AgentId, &Position, &mut Score)>,
    mut grid: ResMut<GridWorld>,
) {
    for (id, pos, mut score) in &mut agents {
        if grid.collect_at(pos.x, pos.y) {
            score.0 += 1.0;
            println!("Agent {} collected resource at ({},{}) — score: {}", id.0, pos.x, pos.y, score.0);
        }
    }
}