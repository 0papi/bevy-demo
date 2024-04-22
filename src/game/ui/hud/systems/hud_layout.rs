use bevy::prelude::*;

use crate::game::ui::hud::hud_components::{ HudDisplay, RedBallDisplay, StarDisplay };

pub fn spawn_star_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _entity = build_star_hud(&mut commands, &asset_server);
}

pub fn build_star_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let entity = commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Local(1),
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    // height: Val::Percent(100.0),
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            HudDisplay {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        background_color: Color::rgb(0.0, 0.0, 0.14).into(),
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,

                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                left: Val::Px(10.0),
                                top: Val::Px(5.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..default()
                    },
                    StarDisplay {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: asset_server.load("sprites/star.png").into(),
                        style: Style {
                            margin: UiRect {
                                right: Val::Px(8.0),
                                ..default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "4".to_string(),
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
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        background_color: Color::rgb(0.0, 0.0, 0.14).into(),
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect {
                                right: Val::Px(10.0),
                                top: Val::Px(5.0),
                                ..Default::default()
                            },

                            ..Default::default()
                        },
                        ..default()
                    },
                    RedBallDisplay {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: asset_server.load("sprites/ball_red_small.png").into(),
                        style: Style {
                            margin: UiRect {
                                right: Val::Px(8.0),
                                ..default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "6".to_string(),
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

    entity
}
