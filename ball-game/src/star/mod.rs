use bevy::prelude::*;

use self::{resources::StarSpawnTimer, systems::*};

pub mod components;
pub mod resources;
mod systems;

pub const STAR_SIZE: f32 = 30.0;
const NUMBER_OF_STARS: usize = 10;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, tick_star_spawn_timer)
            .add_systems(Update, spawn_stars_over_time);
    }
}
