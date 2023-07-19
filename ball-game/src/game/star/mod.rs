use bevy::prelude::*;

use crate::AppState;

use self::{resources::StarSpawnTimer, systems::*};

use super::SimulationState;

pub mod components;
pub mod resources;
mod systems;

pub const STAR_SIZE: f32 = 30.0;
const NUMBER_OF_STARS: usize = 10;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::InGame), spawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_stars);
    }
}
