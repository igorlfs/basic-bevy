use crate::{events::*, game::SimulationState, AppState};
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.get() != &AppState::InGame {
        next_app_state.set(AppState::InGame);
        println!("Entered AppState::InGame");
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.get() != &AppState::MainMenu {
        next_app_state.set(AppState::MainMenu);
        next_simulation_state.set(SimulationState::Paused);
    }
}

pub fn get_bounds(window: &Window, size: f32) -> (f32, f32, f32, f32) {
    let half_player_size = size / 2.0;
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min = 0.0 + half_player_size;
    let y_max = window.height() - half_player_size;
    (x_min, x_max, y_min, y_max)
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score);
        app_state_next_state.set(AppState::GameOver);
    }
}
