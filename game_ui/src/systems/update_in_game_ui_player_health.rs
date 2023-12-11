use bevy::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn update_in_game_ui_player_health(
    mut bar_query: Query<&mut Style, With<PlayerHealthBar>>,
    mut label_query: Query<&mut Text, With<PlayerHealthLabel>>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    let Ok(health) = player_query.get_single() else {
        return;
    };

    if let Ok(mut style) = bar_query.get_single_mut() {
        style.width = Val::Percent(if health.current <= 0 {
            0.
        } else if health.current >= health.max {
            100.
        } else {
            health.current as f32 * 100. / health.max as f32
        });
    }
    if let Ok(mut text) = label_query.get_single_mut() {
        text.sections[0].value = format!("Health: {} / {}", health.current, health.max);
    }
}
