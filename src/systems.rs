use bevy::{ app::AppExit, prelude::*, window::PrimaryWindow };

use crate::{ event::GameOver, game::SimulationState, AppState };

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        match app_state.get() {
            AppState::Game => {}
            _ => {
                commands.insert_resource(NextState(Some(AppState::Game)));
                println!("Transitioned to game state");
            }
        }
    }
}
pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        match app_state.get() {
            AppState::MainMenu => {}
            _ => {
                commands.insert_resource(NextState(Some(AppState::MainMenu)));
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
                println!("Transitioned to main menu state");
            }
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exist_event_writer: EventWriter<AppExit>
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exist_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut commands: Commands, mut game_over_reader: EventReader<GameOver>) {
    for _event in game_over_reader.read() {
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        // println!("Your final score is: {}", event.score.to_string());
    }
}
