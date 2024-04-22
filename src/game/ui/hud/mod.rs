use bevy::prelude::*;

pub mod systems;
pub mod hud_components;
pub mod hud_styles;

use systems::*;

use crate::AppState;

use self::hud_layout::spawn_star_hud;

// TODO: Create HUD Plugin
pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_star_hud.run_if(in_state(AppState::Game)));
    }
}
