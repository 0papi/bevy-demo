use bevy::prelude::*;

pub const ENEMY_SPAWN_TIMER: f32 = 5.0;

#[derive(Resource)]
pub struct SpawnEnemyTimer {
    pub timer: Timer,
}

impl Default for SpawnEnemyTimer {
    fn default() -> Self {
        SpawnEnemyTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}
