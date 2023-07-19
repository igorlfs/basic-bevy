pub mod components;
pub mod resources;
mod systems;
use bevy::prelude::*;

use crate::AppState;

use self::{resources::EnemySpawnTimer, systems::*};

use super::SimulationState;

const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::InGame), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_enemies);
    }
}
