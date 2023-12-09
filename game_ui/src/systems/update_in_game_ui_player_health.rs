use bevy::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn update_in_game_ui_player_health(
    mut label_query: Query<&mut Text, With<PlayerHealthLabel>>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    if let Ok(health) = player_query.get_single() {
        if let Ok(mut text) = label_query.get_single_mut() {
            let color = if health.current >= health.max {
                Color::GREEN
            } else if health.current * 2 >= health.max {
                Color::YELLOW
            } else {
                Color::RED
            };

            text.sections[1].value = format!("{}", health.current);
            text.sections[1].style.color = color;
            text.sections[3].value = format!("{}", health.max);
        }
    }
}
