use bevy::prelude::*;
use game_core::prelude::*;

use crate::prelude::*;

pub fn map_pos_to_trans(mut query: Query<(&MapPos, &mut Transform), Changed<MapPos>>) {
    for (map_pos, mut trans) in &mut query {
        trans.translation.x = map_pos.0.x as f32 * MAIN_TEXTURE_TILE_WIDTH;
        trans.translation.y = map_pos.0.y as f32 * MAIN_TEXTURE_TILE_HEIGHT;
    }
}
