use bevy::{ prelude::*, window::PrimaryWindow };

use crate::{
    event::GameOver,
    game::{ enemy::component::Enemy, score::resource::Score, star::component::Star },
};

use super::component::*;

pub const PLAYER_SIZE: f32 = 64.0; // player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const STAR_SIZE: f32 = 30.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub fn spawn_player(
    mut command: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    let _player = command
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/hole_large.png"),
            ..Default::default()
        })
        .insert(Player {});
}

//Despawn Player
pub fn despawn_player(mut command: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        command.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut command: Commands,
    mut game_over_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>
) {
    if let Ok((_player_entity, player_transform)) = player_query.get_single_mut() {
        let mut collision = false;
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                collision = true;
                let explosion_sound_effect: Handle<AudioSource> = asset_server.load(
                    "audio/impactPunch_heavy_004.ogg"
                );

                println!("Enemy hit player. Game over!");

                command.spawn(AudioBundle {
                    source: explosion_sound_effect,
                    ..default()
                });
            }
        }

        if collision {
            command.entity(_player_entity).despawn();
            game_over_writer.send(GameOver { score: score.value });
        }
    }
}

pub fn player_hit_star(
    mut command: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let mut collision = false;
            let distance = player_transform.translation.distance(star_transform.translation);

            let star_radius = STAR_SIZE / 2.0;
            let player_radius = PLAYER_SIZE / 2.0;

            if distance < player_radius + star_radius {
                collision = true;
                score.value += 1;
                let explosion_sound_effect: Handle<AudioSource> =
                    asset_server.load("audio/minimize_006.ogg");

                command.spawn(AudioBundle {
                    source: explosion_sound_effect,
                    ..default()
                });
            }

            if collision {
                command.entity(star_entity).despawn();
            }
        }
    }
}
