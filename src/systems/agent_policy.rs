use bevy::prelude::*;
use crate::components::{Action, AgentId};

pub fn assign_actions(mut query: Query<(&AgentId, &mut Action)>) {
    for (id, mut action) in &mut query {
        *action = Action::random();
        let _ = id; // swap in real policy later
    }
}