use bevy::prelude::*;
use resources::*;
use systems::*;

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Update, update_score)
            .add_systems(Update, high_scores_updated)
            .add_systems(Update, update_high_scores);
    }
}
