use bevy::prelude::*;
use resources::*;
use systems::*;

use crate::AppState;

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), insert_score)
            .init_resource::<HighScores>()
            .add_systems(Update, update_score.run_if(in_state(AppState::InGame)))
            .add_systems(Update, high_scores_updated)
            .add_systems(Update, update_high_scores)
            .add_systems(OnExit(AppState::InGame), remove_score);
    }
}
