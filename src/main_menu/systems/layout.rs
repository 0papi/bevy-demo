use bevy::prelude::*;

use crate::main_menu::{
    components::{ MainMenu, PlayButton, QuitButton },
    styles::{ create_image_style, create_normal_button_styles, NORMAL_BUTTON_COLOUR },
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive(); // despawn current entity with all its children
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
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
            MainMenu {},
        ))
        .with_children(|parent| {
            let normal_button_styles = create_normal_button_styles();
            let image_styles = create_image_style();
            // ===== TITLE ====
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: image_styles.clone(),
                        image: asset_server.load("sprites/ball_blue_small.png").into(),

                        ..Default::default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Fun Ball Game".to_string(),
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
                    parent.spawn(ImageBundle {
                        style: image_styles.clone(),
                        image: asset_server.load("sprites/ball_red_small.png").into(),

                        ..Default::default()
                    });
                });

            // ===== PLAY ====
            parent
                .spawn((
                    ButtonBundle {
                        style: normal_button_styles.clone(),
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..Default::default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Play".to_string(),
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
            // ===== QUIT ====
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

    main_menu_entity
}
