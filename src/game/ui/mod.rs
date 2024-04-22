use bevy::app::{ App, Plugin };

mod game_over_menu;
mod hud;
mod pause_menu;

use game_over_menu::*;
use pause_menu::*;

use self::hud::HUDPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HUDPlugin).add_plugins(GameOverPlugin).add_plugins(PauseMenuPlugin);
    }
}
