use bevy::prelude::*;
use sha2::{Sha256, Digest};
use crate::components::{AgentId, Position, Score};
use crate::resources::{EpisodeState, EpisodeResult, GridWorld};
use crate::events::EpisodeEnd;
use crate::qtable::QTable;

pub fn tick_episode(
    mut state: ResMut<EpisodeState>,
    mut grid: ResMut<GridWorld>,
    mut end_events: EventWriter<EpisodeEnd>,
    mut agents: Query<(&AgentId, &Position, &mut Score)>,
    mut qtable: ResMut<QTable>,
) {
    if state.done { return; }

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

        // Q-learning: end-of-episode terminal reward
        // Agent 0's final score is the terminal reward signal
        let agent0_score = scores.iter()
            .find(|(id, _)| *id == 0)
            .map(|(_, s)| *s)
            .unwrap_or(0.0);

        let agent1_score = scores.iter()
            .find(|(id, _)| *id == 1)
            .map(|(_, s)| *s)
            .unwrap_or(0.0);

        // bonus reward if agent 0 beat agent 1
        let win_bonus = if agent0_score > agent1_score { 2.0 } else { 0.0 };

        if let (Some(state_s), Some(action_i)) =
            (qtable.last_state, qtable.last_action_idx)
        {
            qtable.update(state_s, action_i, win_bonus, (0, 0));
        }

        qtable.end_episode();

        println!("\n--- Episode {} complete ---", state.episode_id);
        for (id, score) in &scores {
            println!("  Agent {}: {:.1} points", id, score);
        }
        println!("  Episode hash: {}", hash);

        // show Q-learning progress every 10 episodes
        if qtable.episode_count % 10 == 0 && qtable.episode_count > 0 {
            let recent: Vec<f32> = qtable.total_rewards
                .iter().rev().take(10).copied().collect();
            let avg = recent.iter().sum::<f32>() / recent.len() as f32;
            println!("  [Q] ep {} — A0 avg reward last 10: {:.2} — Q-table: {} entries",
                qtable.episode_count, avg, qtable.table.len());
        }

        println!("  → ready to commit on-chain\n");

        end_events.send(EpisodeEnd {
            episode_id: state.episode_id,
            scores,
            ticks: state.tick,
        });

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
