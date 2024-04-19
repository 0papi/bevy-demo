use bevy::prelude::*;

pub mod component;
pub mod resource;
mod systems;

use resource::*;

use crate::AppState;

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnEnemyTimer>()
            .add_systems(OnEnter(AppState::Game), systems::spawn_enemies)
            .add_systems(
                Update,
                systems::enemy_movement
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                Update,
                systems::confine_enemy_movement
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                Update,
                systems::update_enemy_movement
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                Update,
                systems::update_enemy_audio
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                Update,
                systems::tick_enemy_spawn_timer
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                Update,
                systems::spawn_enemy_over_time
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), systems::despawn_enemies);
    }
}
