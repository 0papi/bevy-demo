use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::{
    exit_game,
    handle_game_over,
    spawn_camera,
    transition_to_game_state,
    transition_to_main_menu_state,
};

mod systems;
pub mod event;

mod game;
mod main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)

        .add_systems(Startup, spawn_camera)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)

        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)

        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
