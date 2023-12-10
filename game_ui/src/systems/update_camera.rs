use bevy::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

#[allow(clippy::needless_pass_by_value)]
pub fn update_camera(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
) {
    if let Ok(player_trans) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_trans.translation.x;
            camera_transform.translation.y = player_trans.translation.y;
        }
    }
}
