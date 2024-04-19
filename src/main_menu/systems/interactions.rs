use bevy::prelude::*;
use bevy::app::AppExit;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;
use crate::AppState;

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>)
    >,
    mut app_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state.set(AppState::Game)
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOUR.into();
            }
        }
    }
}
pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>)
    >,
    mut app_exit_event_writer: EventWriter<AppExit>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOUR.into();
            }
        }
    }
}
