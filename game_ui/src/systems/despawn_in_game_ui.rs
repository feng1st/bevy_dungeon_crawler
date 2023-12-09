use bevy::prelude::*;

use crate::prelude::*;

pub fn despawn_in_game_ui(
    mut commands: Commands,
    query: Query<Entity, With<RootNode>>,
    mut tooltip_style_query: Query<&mut Style, With<Tooltip>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }

    for mut tooltip_style in &mut tooltip_style_query {
        tooltip_style.display = Display::None;
    }
}
