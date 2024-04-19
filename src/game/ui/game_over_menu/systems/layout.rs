use bevy::prelude::*;

use crate::{
    event::GameOver,
    game::ui::{
        component::{ FinalScoreText, GameOverMenu, MainMenuButton, QuitButton, ReplayButton },
        styles::{ create_normal_button_styles, NORMAL_BUTTON_COLOUR },
    },
};

/*
this will show the game over screen which should contain the following
  - text at the top showing game over
  - player's score
  - restart or play again button
  - quit button
 */
pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _game_over_menu_entity = build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commads: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_menu_entity = commads
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    ..Default::default()
                },
                ..Default::default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            let normal_button_styles = create_normal_button_styles();

            // ==== Game Over Title ====
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Game Over!".to_string(),
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

            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("Your final score was:", TextStyle {
                                font_size: 32.0,
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                color: Color::WHITE,
                            })
                        ],
                        ..default()
                    },
                    ..default()
                },
                FinalScoreText {},
            ));

            // ==== Score ====
            // parent
            //     .spawn(NodeBundle {
            //         style: Style {
            //             flex_direction: FlexDirection::Row,
            //             justify_content: JustifyContent::Center,
            //             align_items: AlignItems::Center,
            //             column_gap: Val::Px(40.0),
            //             ..Default::default()
            //         },
            //         ..Default::default()
            //     })
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle {
            //             text: Text {
            //                 sections: vec![TextSection {
            //                     value: "Score:".to_string(),
            //                     style: TextStyle {
            //                         font_size: 32.0,
            //                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //                         color: Color::WHITE,
            //                     },
            //                 }],
            //                 justify: JustifyText::Center,
            //                 ..Default::default()
            //             },
            //             ..Default::default()
            //         });
            //         parent.spawn((
            //             TextBundle {
            //                 text: Text {
            //                     sections: vec![
            //                         TextSection::new("0", TextStyle {
            //                             font_size: 32.0,
            //                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //                             color: Color::WHITE,
            //                         })
            //                     ],
            //                     justify: JustifyText::Center,
            //                     ..Default::default()
            //                 },
            //                 ..Default::default()
            //             },
            //             FinalScoreText {},
            //         ));
            //     });
            // ==== Highest Score ====
            // parent
            //     .spawn(NodeBundle {
            //         style: Style {
            //             flex_direction: FlexDirection::Row,
            //             justify_content: JustifyContent::Center,
            //             align_items: AlignItems::Center,
            //             column_gap: Val::Px(40.0),
            //             ..Default::default()
            //         },
            //         ..Default::default()
            //     })
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle {
            //             text: Text {
            //                 sections: vec![TextSection {
            //                     value: "Best:".to_string(),
            //                     style: TextStyle {
            //                         font_size: 32.0,
            //                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //                         color: Color::WHITE,
            //                     },
            //                 }],
            //                 justify: JustifyText::Center,
            //                 ..Default::default()
            //             },
            //             ..Default::default()
            //         });
            //         parent.spawn((
            //             TextBundle {
            //                 text: Text {
            //                     sections: vec![TextSection {
            //                         value: (10).to_string(),
            //                         style: TextStyle {
            //                             font_size: 32.0,
            //                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //                             color: Color::WHITE,
            //                         },
            //                     }],
            //                     justify: JustifyText::Center,
            //                     ..Default::default()
            //                 },
            //                 ..Default::default()
            //             },
            //             HighestScoreText {},
            //         ));
            //     });

            // ==== Replay & Quit ====
            // ===== PLAY ====
            parent
                .spawn((
                    ButtonBundle {
                        style: normal_button_styles.clone(),
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..Default::default()
                    },
                    ReplayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Replay".to_string(),
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
        })
        .id();

    game_over_menu_entity
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_query: Query<Entity, With<GameOverMenu>>
) {
    if let Ok(game_over_entity) = game_over_query.get_single() {
        commands.entity(game_over_entity).despawn_recursive();
    }
}
