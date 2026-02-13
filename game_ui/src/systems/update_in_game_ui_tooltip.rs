use std::fmt::Write;

use bevy::{prelude::*, window::PrimaryWindow};
use game_core::prelude::{Name, *};

use crate::prelude::*;

pub fn update_in_game_ui_tooltip(
    mut tooltip_style_query: Query<&mut Style, With<Tooltip>>,
    mut tooltip_text_query: Query<&mut Text, With<TooltipText>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    cursor_position: Res<CursorPosition>,
    player_query: Query<&FieldOfView, With<Player>>,
    entity_query: Query<(&MapPos, &Name, Option<&Health>), Without<Player>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let Ok(mut tooltip_style) = tooltip_style_query.get_single_mut() else {
        return;
    };

    let cursor_pos = IVec2::new(
        (cursor_position.world.x / MAIN_TEXTURE_TILE_WIDTH).round() as i32,
        (cursor_position.world.y / MAIN_TEXTURE_TILE_HEIGHT).round() as i32,
    );

    let mut point: Option<Vec2> = None;
    if let Ok(mut tooltip_text) = tooltip_text_query.get_single_mut() {
        if let Ok(field_of_view) = player_query.get_single() {
            for (map_pos, name, health) in entity_query.iter() {
                if map_pos.0 == cursor_pos && field_of_view.visible_tiles.contains(&map_pos.0) {
                    let mut text = name.0.to_string();
                    if let Some(h) = health {
                        let _ = write!(text, "\nHealth: {} / {}", h.current, h.max);
                    }
                    tooltip_text.sections[0].value = text;
                    point = Some(Vec2::new(
                        cursor_position.viewport.x,
                        window.height() - cursor_position.viewport.y,
                    ));
                    break;
                }
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
