use bevy::prelude::*;

use crate::game::ui::{
    pause_components::{ PauseMenu, QuitButton, MainMenuButton, ResumeButton },
    pause_menu_styles::{ create_normal_button_styles, NORMAL_BUTTON_COLOUR },
};

// spawn pause menu layout
pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_entity = commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Local(1),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    position_type: PositionType::Absolute,

                    ..default()
                },
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    z_index: ZIndex::Local(1),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        width: Val::Percent(30.0),
                        height: Val::Percent(40.0),
                        display: Display::Flex,
                        padding: UiRect {
                            left: Val::Px(4.0),
                            right: Val::Px(4.0),
                            top: Val::Px(4.0),
                            bottom: Val::Px(4.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let normal_button_styles = create_normal_button_styles();
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Pause Menu".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            }],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });

                    // ==== Resume
                    parent
                        .spawn((
                            ButtonBundle {
                                style: normal_button_styles.clone(),
                                background_color: NORMAL_BUTTON_COLOUR.into(),
                                ..Default::default()
                            },
                            ResumeButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Resume".to_string(),
                                        style: TextStyle {
                                            font_size: 32.0,
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            color: Color::WHITE,
                                        },
                                    }],
                                    justify: JustifyText::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                    // ==== Main Menu
                    parent
                        .spawn((
                            ButtonBundle {
                                style: normal_button_styles.clone(),
                                background_color: NORMAL_BUTTON_COLOUR.into(),
                                ..Default::default()
                            },
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Main Menu".to_string(),
                                        style: TextStyle {
                                            font_size: 32.0,
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            color: Color::WHITE,
                                        },
                                    }],
                                    justify: JustifyText::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });

                    //  ===== QUIT ====
                    parent
                        .spawn((
                            ButtonBundle {
                                style: normal_button_styles.clone(),
                                background_color: NORMAL_BUTTON_COLOUR.into(),
                                ..Default::default()
                            },
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Quit".to_string(),
                                        style: TextStyle {
                                            font_size: 32.0,
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            color: Color::WHITE,
                                        },
                                    }],
                                    justify: JustifyText::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                });
        })

        .id();
    pause_entity
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive(); // despawn current entity with all its children
    }
}
