use bevy::prelude::*;
use game_core::prelude::{Name, *};

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::cast_possible_truncation)]
pub fn update_in_game_ui_tooltip(
    mut tooltip_style_query: Query<&mut Style, With<Tooltip>>,
    mut tooltip_text_query: Query<&mut Text, With<TooltipText>>,
    cursor_position: Res<CursorPosition>,
    map_query: Query<&MapFigureGrid>,
    monster_query: Query<(&Name, &Health), With<Monster>>,
) {
    let map_pos = IVec2::new(
        (cursor_position.world.x / MAIN_TEXTURE_TILE_WIDTH).round() as i32,
        (cursor_position.world.y / MAIN_TEXTURE_TILE_HEIGHT).round() as i32,
    );

    if let Ok(mut tooltip_style) = tooltip_style_query.get_single_mut() {
        if let Ok(map_figure_grid) = map_query.get_single() {
            if let Ok((name, health)) = monster_query.get(map_figure_grid.get(map_pos)) {
                if let Ok(mut tooltip_text) = tooltip_text_query.get_single_mut() {
                    tooltip_text.sections[0].value =
                        format!("{} - {} / {}", name.0, health.current, health.max);
                }
                tooltip_style.display = Display::Flex;
                tooltip_style.left = Val::Px(cursor_position.viewport.x);
                tooltip_style.top = Val::Px(cursor_position.viewport.y - 20.0);
            } else {
                tooltip_style.display = Display::None;
            }
        } else {
            tooltip_style.display = Display::None;
        }
    }
}
