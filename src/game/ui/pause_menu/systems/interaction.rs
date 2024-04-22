use bevy::{ app::AppExit, prelude::* };

use crate::{
    game::{
        ui::{
            pause_components::{ MainMenuButton, QuitButton, ResumeButton },
            pause_menu_styles::{ HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOUR, PRESSED_BUTTON_COLOR },
        },
        SimulationState,
    },
    AppState,
};

pub fn interact_with_resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>)
    >,
    mut app_state: ResMut<NextState<SimulationState>>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state.set(SimulationState::Running);
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOUR.into();
            }
        }
    }
}

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>)
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
                app_state.set(AppState::MainMenu)
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
