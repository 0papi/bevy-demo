use bevy::prelude::*;

use crate::event::GameOver;

use super::resource::{ HighScores, Score };

pub fn insert_score(mut command: Commands) {
    command.insert_resource(Score::default())
}

pub fn remove_score(mut command: Commands) {
    command.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}

pub fn update_high_scores(
    mut game_over_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScores>
) {
    for event in game_over_reader.read() {
        high_score.scores.push((String::from("Player"), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
