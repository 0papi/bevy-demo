use bevy::prelude::*;

use crate::game::SimulationState;

pub mod systems;
pub mod pause_components;
pub mod pause_menu_styles;

use systems::*;

use self::interaction::{
    interact_with_main_menu_button,
    interact_with_quit_button,
    interact_with_resume_button,
};
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), layout::spawn_pause_menu)
            .add_systems(
                Update,
                (
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                    interact_with_resume_button,
                ).run_if(in_state(SimulationState::Paused))
            )

            .add_systems(OnExit(SimulationState::Paused), layout::despawn_pause_menu);
    }
}
