use bevy::prelude::*;

use crate::{ event::GameOver, game::ui::component::FinalScoreText };

pub fn update_final_score_text(
    mut game_over_event_reader: EventReader<GameOver>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>
) {
    println!("Updating final score text...");
    for event in game_over_event_reader.read() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.score.to_string());
        }
    }
}
