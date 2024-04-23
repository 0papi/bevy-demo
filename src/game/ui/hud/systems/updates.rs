use crate::game::{
    enemy::component::Enemy,
    score::resource::Score,
    ui::hud::hud_components::{ RedBallDisplay, StarDisplay },
};
use bevy::prelude::*;

pub fn update_score_text(mut text_query: Query<&mut Text, With<StarDisplay>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = score.value.to_string();
        }
    }
}

pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<RedBallDisplay>>,
    enemy_query: Query<Entity, With<Enemy>>
) {
    let count = enemy_query.iter().count();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = count.to_string();
    }
}
