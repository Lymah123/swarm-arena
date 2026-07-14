use bevy::prelude::*;
use crate::components::{AgentId, Position, Score};
use crate::resources::GridWorld;
use crate::qtable::QTable;
use crate::neural_policy::NeuralPolicy;
use crate::systems::agent_policy::on_resource_collected;

pub fn collect_resources(
    mut agents: Query<(&AgentId, &Position, &mut Score)>,
    mut grid: ResMut<GridWorld>,
    mut qtable: ResMut<QTable>,
    mut neural: ResMut<NeuralPolicy>,
) {
    for (id, pos, mut score) in &mut agents {
        if grid.collect_at(pos.x, pos.y) {
            score.0 += 1.0;
            println!(
                "Agent {} collected resource at ({},{}) — score: {}",
                id.0, pos.x, pos.y, score.0
            );

            let nearest = grid.resources
                .iter()
                .min_by_key(|(rx, ry)| (rx - pos.x).abs() + (ry - pos.y).abs())
                .copied()
                .unwrap_or((pos.x, pos.y));

            let walls_clone = grid.walls.clone();
            on_resource_collected(
                id.0,
                pos.x, pos.y,
                nearest.0, nearest.1,
                &mut qtable,
                &mut neural,
                grid.width,
                grid.height,
                &walls_clone,
            );
        }
    }
}
