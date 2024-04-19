use bevy::prelude::*;

use crate::AppState;

mod systems;
pub mod styles;

pub mod component;

use systems::*;

use self::{
    interactions::{
        interact_with_main_menu_button,
        interact_with_quit_button,
        interact_with_replay_button,
    },
    layout::{ despawn_game_over_menu, spawn_game_over_menu },
    updates::update_final_score_text,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(
                Update,
                (
                    interact_with_replay_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                ).run_if(in_state(AppState::GameOver))
            )
            .add_systems(Update, update_final_score_text.run_if(in_state(AppState::GameOver)))
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
