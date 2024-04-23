use bevy::prelude::*;

pub mod systems;
pub mod hud_components;
pub mod hud_styles;

use systems::*;

use crate::AppState;

use self::{
    hud_layout::{ despawn_hud, spawn_star_hud },
    updates::{ update_enemy_text, update_score_text },
};

// TODO: Create HUD Plugin
pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_star_hud.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                (update_score_text, update_enemy_text).run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}
