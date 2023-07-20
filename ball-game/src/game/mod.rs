mod enemy;
mod player;
mod score;
mod star;
mod systems;

use crate::{events::GameOver, AppState};
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_systems(OnEnter(AppState::InGame), pause_simulation)
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
