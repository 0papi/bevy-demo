use bevy::app::{ App, Plugin };

mod game_over_menu;
mod hud;
mod pause_menu;

use game_over_menu::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameOverPlugin);
    }
}
