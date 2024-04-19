use bevy::prelude::*;

pub mod resource;
mod systems;

use systems::*;
use resource::*;

use crate::AppState;
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), insert_score)
            .init_resource::<HighScores>()
            .add_systems(Update, update_score.run_if(in_state(AppState::Game)))
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated)
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
