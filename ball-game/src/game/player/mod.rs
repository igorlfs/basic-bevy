use crate::AppState;

use self::systems::*;
use bevy::prelude::*;

use super::SimulationState;

pub mod components;
mod systems;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct MovementSystemSet;
//
// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct ConfinementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement),
        )
        .add_systems(OnEnter(AppState::InGame), spawn_player)
        .add_systems(
            Update,
            (
                player_movement.in_set(PlayerSystemSet::Movement),
                confine_player_movement.in_set(PlayerSystemSet::Confinement),
            )
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(SimulationState::Running)),
        )
        .add_systems(
            Update,
            (player_hit_stars, player_hit_enemies)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(SimulationState::Running)),
        )
        .add_systems(OnExit(AppState::InGame), despawn_player);
    }
}
