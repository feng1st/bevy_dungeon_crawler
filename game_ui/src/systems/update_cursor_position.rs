use bevy::{prelude::*, window::PrimaryWindow};

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Some(viewport_pos) = window_query
        .get_single()
        .ok()
        .and_then(Window::cursor_position)
    else {
        return;
    };

    if let Some(world_pos) = camera_query
        .get_single()
        .ok()
        .and_then(|(camera, camera_trans)| camera.viewport_to_world(camera_trans, viewport_pos))
        .map(|ray| ray.origin.truncate())
    {
        cursor_pos.viewport = viewport_pos;
        cursor_pos.world = world_pos;
    }
}
