use bevy::{ prelude::*, window::PrimaryWindow };
use rand::random;

use super::{ component::Enemy, resource::SpawnEnemyTimer };

// ENEMY CONSTANTS
pub const ENEMY_SIZE: f32 = 64.0; // player sprite size

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;

// spawn enemies here
pub fn spawn_enemies(
    mut command: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let _enemy = command
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            })
            .insert(Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            });
    }
}

// Despawn Enemies
pub fn despawn_enemies(mut command: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        command.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

        //move enemy
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0; // 32
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
    }
}
pub fn update_enemy_audio(
    mut command: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, _enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        let mut direction_change = false;

        if translation.x < x_min || translation.x > x_max {
            direction_change = true;
        }
        if translation.y < y_min || translation.y > y_max {
            direction_change = true;
        }

        if direction_change {
            let sound_effect_1 = asset_server.load("audio/impactMetal_heavy_000.ogg");
            let sound_effect_2 = asset_server.load("audio/impactMetal_heavy_001.ogg");

            let sound_effect = if random::<f32>() > 0.5 { sound_effect_1 } else { sound_effect_2 };

            command.spawn(AudioBundle {
                source: sound_effect,
                ..default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut enemy_transform in enemy_query.iter_mut() {
        let mut translation = enemy_transform.translation;

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
        enemy_transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(mut spawn_enemy_timer: ResMut<SpawnEnemyTimer>, time: Res<Time>) {
    spawn_enemy_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_over_time(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemy_spawn: Res<SpawnEnemyTimer>
) {
    if enemy_spawn.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let _star = command
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            })
            .insert(Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            });
    }
}
