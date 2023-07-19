use self::systems::*;
use bevy::prelude::*;

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
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement.in_set(PlayerSystemSet::Movement))
        .add_systems(
            Update,
            confine_player_movement.in_set(PlayerSystemSet::Confinement),
        )
        // .add_systems(Update, (player_movement, confine_player_movement).chain())
        .add_systems(Update, player_hit_stars)
        .add_systems(Update, player_hit_enemies);
    }
}
