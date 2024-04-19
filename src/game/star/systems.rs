use bevy::{ prelude::*, window::PrimaryWindow };
use rand::random;

use super::{ component::Star, resources::StarSpawnTimer };

pub const NUMBER_OF_STARS: usize = 10;

pub fn spawn_stars(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let _star = command
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..Default::default()
            })
            .insert(Star {});
    }
}

// Despawn Stars
pub fn despawn_stars(mut command: Commands, star_query: Query<Entity, With<Star>>) {
    for star_entity in star_query.iter() {
        command.entity(star_entity).despawn();
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    star_spawn_timer: Res<StarSpawnTimer>
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let _star = command
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..Default::default()
            })
            .insert(Star {});
    }
}
