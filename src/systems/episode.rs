use bevy::prelude::*;
use sha2::{Sha256, Digest};
use crate::components::{AgentId, Position, Score};
use crate::resources::{EpisodeState, EpisodeResult, GridWorld};
use crate::events::EpisodeEnd;

pub fn tick_episode(
    mut state: ResMut<EpisodeState>,
    mut grid: ResMut<GridWorld>,
    mut end_events: EventWriter<EpisodeEnd>,
    mut agents: Query<(&AgentId, &Position, &mut Score)>,
) {
    if state.done {
        return;
    }

    state.tick += 1;

    if state.tick >= state.max_ticks {
        state.done = true;

        let scores: Vec<(u8, f32)> = agents
            .iter()
            .map(|(id, _, score)| (id.0, score.0))
            .collect();

        let result = EpisodeResult {
            episode_id: state.episode_id,
            agent_scores: scores.clone(),
            ticks: state.tick,
        };

        let hash = hash_episode(&result);

        println!("\n--- Episode {} complete ---", state.episode_id);
        for (id, score) in &scores {
            println!("  Agent {}: {:.1} points", id, score);
        }
        println!("  Episode hash: {}", hash);
        println!("  → ready to commit on-chain\n");

        end_events.send(EpisodeEnd {
            episode_id: state.episode_id,
            scores,
            ticks: state.tick,
        });

        // reset for next episode
        state.reset();
        grid.reset();
        for (_, _, mut score) in &mut agents {
            score.0 = 0.0;
        }
    }
}

pub fn hash_episode(result: &EpisodeResult) -> String {
    let json = serde_json::to_string(result).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    format!("{:x}", hasher.finalize())
}