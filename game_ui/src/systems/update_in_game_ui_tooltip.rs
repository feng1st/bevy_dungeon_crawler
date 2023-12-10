use bevy::{prelude::*, window::PrimaryWindow};
use game_core::prelude::{Name, *};

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::cast_possible_truncation)]
pub fn update_in_game_ui_tooltip(
    mut tooltip_style_query: Query<&mut Style, With<Tooltip>>,
    mut tooltip_text_query: Query<&mut Text, With<TooltipText>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    cursor_position: Res<CursorPosition>,
    map_query: Query<&MapFigureGrid>,
    monster_query: Query<(&Name, &Health), With<Monster>>,
) {
    let map_pos = IVec2::new(
        (cursor_position.world.x / MAIN_TEXTURE_TILE_WIDTH).round() as i32,
        (cursor_position.world.y / MAIN_TEXTURE_TILE_HEIGHT).round() as i32,
    );

    if let Ok(window) = window_query.get_single() {
        if let Ok(mut tooltip_style) = tooltip_style_query.get_single_mut() {
            let mut point: Option<Vec2> = None;
            if let Ok(map_figure_grid) = map_query.get_single() {
                if let Ok((name, health)) = monster_query.get(map_figure_grid.get(map_pos)) {
                    if let Ok(mut tooltip_text) = tooltip_text_query.get_single_mut() {
                        tooltip_text.sections[0].value =
                            format!("{}\nHealth: {} / {}", name.0, health.current, health.max);

                        point = Some(Vec2::new(
                            cursor_position.viewport.x,
                            window.height() - cursor_position.viewport.y,
                        ));
                    }
                }
            }

            if let Some(point) = point {
                tooltip_style.display = Display::Flex;
                tooltip_style.left = Val::Px(point.x);
                tooltip_style.bottom = Val::Px(point.y);
            } else {
                tooltip_style.display = Display::None;
            }
        }
    }
}
