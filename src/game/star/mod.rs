use bevy::prelude::*;

mod systems;
pub mod component;
pub mod resources;

use systems::*;
use resources::*;

use crate::AppState;

use super::SimulationState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                tick_star_spawn_timer
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                Update,
                spawn_stars_over_time
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), systems::despawn_stars);
    }
}
