use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_amulet_of_yala(
    //mut commands: Commands,
    pos_query: Query<&AmuletStart>,
    mut map_query: Query<&mut MapTileGrid>,
) {
    let Ok(amulet_start) = pos_query.get_single() else {
        return;
    };

    let Ok(mut map_tile_grid) = map_query.get_single_mut() else {
        return;
    };

    map_tile_grid.set_at(amulet_start.0, TileType::Exit);

    //spawn_item(&mut commands, 0, amulet_start.0);
}
